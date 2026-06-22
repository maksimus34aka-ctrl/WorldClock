#!/usr/bin/env python3
# world_clock.py - Мировые часы на Python (Tkinter)
import tkinter as tk
from tkinter import ttk, messagebox
import json
import os
from datetime import datetime, timedelta
import time

# База городов с часовыми поясами (смещение от UTC в часах)
CITIES = {
    "Лондон": 0,
    "Париж": 1,
    "Москва": 3,
    "Дубай": 4,
    "Нью-Йорк": -5,
    "Лос-Анджелес": -8,
    "Токио": 9,
    "Пекин": 8,
    "Сидней": 11,
    "Нью-Дели": 5.5,
    "Сан-Паулу": -3,
    "Йоханнесбург": 2,
    "Сингапур": 8,
    "Гонконг": 8,
    "Стамбул": 3,
    "Рим": 1,
    "Берлин": 1,
    "Киев": 3,
    "Минск": 3,
    "Алматы": 6,
    "Новосибирск": 7,
    "Владивосток": 10,
    "Магадан": 11,
    "Камчатка": 12,
    "Лиссабон": 0,
    "Дублин": 0,
    "Афины": 2,
    "Каир": 2,
    "Найроби": 3,
    "Кейптаун": 2,
    "Бангкок": 7,
    "Джакарта": 7,
    "Сеул": 9,
    "Окленд": 13,
    "Гонолулу": -10,
    "Анкоридж": -9,
    "Чикаго": -6,
    "Денвер": -7,
    "Финикс": -7,
    "Майами": -5,
    "Торонто": -5,
    "Ванкувер": -8,
    "Мехико": -6,
    "Буэнос-Айрес": -3,
    "Рио-де-Жанейро": -3,
    "Перth": 8,
    "Аделаида": 10.5,
    "Канберра": 11,
}

DATA_FILE = "world_clock_cities.json"

class WorldClock:
    def __init__(self, root):
        self.root = root
        self.root.title("🌍 Мировые часы - Python")
        self.root.geometry("700x500")
        self.root.resizable(True, True)

        self.cities = self.load_cities()
        self.format_24h = True
        self.running = True

        self.create_widgets()
        self.update_clocks()

    def load_cities(self):
        if os.path.exists(DATA_FILE):
            try:
                with open(DATA_FILE, 'r', encoding='utf-8') as f:
                    return json.load(f)
            except:
                pass
        return ["Москва", "Лондон", "Нью-Йорк", "Токио", "Сидней"]

    def save_cities(self):
        with open(DATA_FILE, 'w', encoding='utf-8') as f:
            json.dump(self.cities, f, ensure_ascii=False, indent=2)

    def create_widgets(self):
        # Верхняя панель
        top = tk.Frame(self.root, bg="#2c3e50", height=60)
        top.pack(fill=tk.X)
        tk.Label(top, text="🌍 Мировые часы", font=('Arial', 20, 'bold'),
                 fg="white", bg="#2c3e50").pack(side=tk.LEFT, padx=20)

        btn_frame = tk.Frame(top, bg="#2c3e50")
        btn_frame.pack(side=tk.RIGHT, padx=10)
        tk.Button(btn_frame, text="Добавить город", command=self.add_city,
                  bg="#3498db", fg="white").pack(side=tk.LEFT, padx=5)
        tk.Button(btn_frame, text="12/24 ч", command=self.toggle_format,
                  bg="#f39c12", fg="white").pack(side=tk.LEFT, padx=5)
        tk.Button(btn_frame, text="Удалить все", command=self.clear_all,
                  bg="#e74c3c", fg="white").pack(side=tk.LEFT, padx=5)

        # Список городов
        self.canvas = tk.Canvas(self.root)
        self.scrollbar = tk.Scrollbar(self.root, orient=tk.VERTICAL, command=self.canvas.yview)
        self.scrollable_frame = tk.Frame(self.canvas)

        self.scrollable_frame.bind(
            "<Configure>",
            lambda e: self.canvas.configure(scrollregion=self.canvas.bbox("all"))
        )

        self.canvas.create_window((0, 0), window=self.scrollable_frame, anchor="nw")
        self.canvas.configure(yscrollcommand=self.scrollbar.set)

        self.canvas.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        self.scrollbar.pack(side=tk.RIGHT, fill=tk.Y)

        # Фреймы для каждого города
        self.city_frames = {}
        self.time_labels = {}
        self.date_labels = {}

        self.build_clock_list()

    def build_clock_list(self):
        # Очищаем старые виджеты
        for widget in self.scrollable_frame.winfo_children():
            widget.destroy()
        self.city_frames.clear()
        self.time_labels.clear()
        self.date_labels.clear()

        for city in self.cities:
            frame = tk.Frame(self.scrollable_frame, relief=tk.RIDGE, bd=2, padx=10, pady=5)
            frame.pack(fill=tk.X, pady=3, padx=5)

            # Название города
            name_label = tk.Label(frame, text=city, font=('Arial', 14, 'bold'), width=15, anchor='w')
            name_label.pack(side=tk.LEFT, padx=5)

            # Время
            time_label = tk.Label(frame, font=('Courier', 20, 'bold'), width=12, anchor='e')
            time_label.pack(side=tk.LEFT, padx=10)

            # Дата
            date_label = tk.Label(frame, font=('Arial', 10), width=20, anchor='w')
            date_label.pack(side=tk.LEFT, padx=5)

            # Кнопка удаления
            del_btn = tk.Button(frame, text="✕", command=lambda c=city: self.remove_city(c),
                               bg="#e74c3c", fg="white", width=3)
            del_btn.pack(side=tk.RIGHT, padx=5)

            self.city_frames[city] = frame
            self.time_labels[city] = time_label
            self.date_labels[city] = date_label

    def update_clocks(self):
        if not self.running:
            return
        now_utc = datetime.utcnow()
        for city in self.cities:
            offset = CITIES.get(city, 0)
            dt = now_utc + timedelta(hours=offset)
            if self.format_24h:
                time_str = dt.strftime("%H:%M:%S")
            else:
                time_str = dt.strftime("%I:%M:%S %p")
            date_str = dt.strftime("%a, %d %b %Y")
            if city in self.time_labels:
                self.time_labels[city].config(text=time_str)
                self.date_labels[city].config(text=date_str)
        self.root.after(1000, self.update_clocks)

    def add_city(self):
        # Диалог выбора города
        dialog = tk.Toplevel(self.root)
        dialog.title("Добавить город")
        dialog.geometry("300x400")
        dialog.grab_set()

        tk.Label(dialog, text="Выберите город:", font=('Arial', 12)).pack(pady=10)

        listbox = tk.Listbox(dialog, selectmode=tk.SINGLE)
        listbox.pack(fill=tk.BOTH, expand=True, padx=10, pady=5)

        for city in sorted(CITIES.keys()):
            if city not in self.cities:
                listbox.insert(tk.END, city)

        def add_selected():
            selection = listbox.curselection()
            if selection:
                city = listbox.get(selection[0])
                self.cities.append(city)
                self.save_cities()
                self.build_clock_list()
                dialog.destroy()
            else:
                messagebox.showwarning("Выберите город", "Пожалуйста, выберите город из списка.")

        tk.Button(dialog, text="Добавить", command=add_selected, bg="#2ecc71", fg="white").pack(pady=10)

    def remove_city(self, city):
        if city in self.cities:
            self.cities.remove(city)
            self.save_cities()
            self.build_clock_list()

    def clear_all(self):
        if messagebox.askyesno("Удалить все", "Удалить все города из списка?"):
            self.cities = []
            self.save_cities()
            self.build_clock_list()

    def toggle_format(self):
        self.format_24h = not self.format_24h

    def on_closing(self):
        self.running = False
        self.root.destroy()

if __name__ == "__main__":
    root = tk.Tk()
    app = WorldClock(root)
    root.protocol("WM_DELETE_WINDOW", app.on_closing)
    root.mainloop()
