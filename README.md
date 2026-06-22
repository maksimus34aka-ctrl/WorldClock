WorldClock Suite — Мировые часы на 7 языках
WorldClock Suite — коллекция из семи независимых реализаций мировых часов с возможностью отображения времени в разных часовых поясах. Каждая версия работает на своём языке программирования и предлагает гибкие настройки для удобного отслеживания времени по всему миру.

✨ Общие возможности
🕐 Отображение текущего времени для нескольких городов/часовых поясов

➕ Добавление/удаление городов из списка (встроенная база из 50+ городов)

🔄 Автоматическое обновление каждую секунду

🎨 Переключение формата 12/24 часа

💾 Сохранение списка городов в файл или localStorage

🖱️ Интерактивный интерфейс (выбор города из списка, удаление кликом)

📅 Отображение даты и дня недели для каждого города (опционально)

🌐 Интерфейсы:

Десктопные GUI: Python (Tkinter), Java (Swing), C# (WinForms)

Веб-приложения: JavaScript (HTML+CSS+JS), Go, Rust, PHP (сервер + клиент)

📋 Сравнение реализаций
Язык	Интерфейс	Сохранение	Добавление городов	12/24 ч	Дата
Python	Tkinter GUI	JSON-файл	✅ (список)	✅	✅
JavaScript	Веб (HTML+CSS)	localStorage	✅ (выбор)	✅	✅
Go	Веб (сервер)	файл (JSON)	✅ (выбор)	✅	✅
Rust	Веб (сервер)	файл (JSON)	✅ (выбор)	✅	✅
Java	Swing GUI	файл	✅ (список)	✅	✅
C#	WinForms GUI	файл	✅ (список)	✅	✅
PHP	Веб (сервер)	сессия/файл	✅ (выбор)	✅	✅
🚀 Быстрый старт
Python
bash
# Tkinter встроен
python world_clock.py
JavaScript (браузер)
Откройте world_clock.html в браузере.

Go
bash
go run world_clock.go
# Откройте http://localhost:8080
Rust
bash
cargo run
# Откройте http://localhost:8000
Java
bash
javac WorldClock.java && java WorldClock
C#
bash
csc /reference:System.Windows.Forms.dll /reference:System.Drawing.dll WorldClock.cs
WorldClock.exe
PHP
bash
php -S localhost:8000
# Откройте http://localhost:8000/world_clock.php
