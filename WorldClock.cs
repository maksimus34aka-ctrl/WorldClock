// WorldClock.cs - Мировые часы на C# (WinForms)
using System;
using System.Collections.Generic;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Windows.Forms;

public class WorldClock : Form
{
    private List<string> cities;
    private bool format24 = true;
    private Timer timer;
    private Panel clockPanel;
    private Dictionary<string, Label[]> clockLabels = new Dictionary<string, Label[]>();
    private const string DATA_FILE = "world_cities.json";

    private static readonly Dictionary<string, double> CitiesMap = new Dictionary<string, double>
    {
        {"Лондон", 0}, {"Париж", 1}, {"Москва", 3}, {"Дубай", 4},
        {"Нью-Йорк", -5}, {"Лос-Анджелес", -8}, {"Токио", 9},
        {"Пекин", 8}, {"Сидней", 11}, {"Нью-Дели", 5.5},
        {"Сан-Паулу", -3}, {"Йоханнесбург", 2}, {"Сингапур", 8},
        {"Гонконг", 8}, {"Стамбул", 3}, {"Рим", 1}, {"Берлин", 1},
        {"Киев", 3}, {"Минск", 3}, {"Алматы", 6}, {"Новосибирск", 7},
        {"Владивосток", 10}, {"Магадан", 11}, {"Камчатка", 12},
        {"Лиссабон", 0}, {"Дублин", 0}, {"Афины", 2}, {"Каир", 2},
        {"Найроби", 3}, {"Кейптаун", 2}, {"Бангкок", 7}, {"Джакарта", 7},
        {"Сеул", 9}, {"Окленд", 13}, {"Гонолулу", -10}, {"Анкоридж", -9},
        {"Чикаго", -6}, {"Денвер", -7}, {"Финикс", -7}, {"Майами", -5},
        {"Торонто", -5}, {"Ванкувер", -8}, {"Мехико", -6}, {"Буэнос-Айрес", -3},
        {"Рио-де-Жанейро", -3}, {"Перт", 8}, {"Аделаида", 10.5}, {"Канберра", 11}
    };

    public WorldClock()
    {
        Text = "🌍 Мировые часы - C#";
        Size = new Size(750, 550);
        StartPosition = FormStartPosition.CenterScreen;

        cities = LoadCities();
        InitUI();
        StartClock();
    }

    private List<string> LoadCities()
    {
        if (File.Exists(DATA_FILE))
        {
            try
            {
                string json = File.ReadAllText(DATA_FILE);
                return JsonSerializer.Deserialize<List<string>>(json) ?? new List<string>();
            }
            catch { }
        }
        return new List<string> { "Москва", "Лондон", "Нью-Йорк", "Токио", "Сидней" };
    }

    private void SaveCities()
    {
        string json = JsonSerializer.Serialize(cities, new JsonSerializerOptions { WriteIndented = true });
        File.WriteAllText(DATA_FILE, json);
    }

    private void InitUI()
    {
        // Верхняя панель
        Panel top = new Panel { Dock = DockStyle.Top, Height = 60, BackColor = Color.FromArgb(44, 62, 80) };
        Label title = new Label { Text = "🌍 Мировые часы", Font = new Font("Arial", 20, FontStyle.Bold), ForeColor = Color.White, Dock = DockStyle.Fill, TextAlign = ContentAlignment.MiddleCenter };
        top.Controls.Add(title);

        FlowLayoutPanel btnPanel = new FlowLayoutPanel { Dock = DockStyle.Right, Height = 60, BackColor = Color.Transparent };
        Button addBtn = new Button { Text = "➕ Добавить город", BackColor = Color.DodgerBlue, ForeColor = Color.White, FlatStyle = FlatStyle.Flat };
        addBtn.Click += (s, e) => AddCity();
        btnPanel.Controls.Add(addBtn);
        Button formatBtn = new Button { Text = "12/24 ч", BackColor = Color.Goldenrod, ForeColor = Color.White, FlatStyle = FlatStyle.Flat };
        formatBtn.Click += (s, e) => { format24 = !format24; };
        btnPanel.Controls.Add(formatBtn);
        Button clearBtn = new Button { Text = "🗑️ Очистить", BackColor = Color.Crimson, ForeColor = Color.White, FlatStyle = FlatStyle.Flat };
        clearBtn.Click += (s, e) => ClearAll();
        btnPanel.Controls.Add(clearBtn);
        top.Controls.Add(btnPanel);

        Controls.Add(top);

        // Список городов
        clockPanel = new Panel { Dock = DockStyle.Fill, AutoScroll = true, BackColor = Color.FromArgb(44, 62, 80) };
        Controls.Add(clockPanel);

        BuildClockList();
    }

    private void BuildClockList()
    {
        clockPanel.Controls.Clear();
        clockLabels.Clear();
        int y = 10;
        foreach (string city in cities)
        {
            Panel item = new Panel { Size = new Size(clockPanel.Width - 20, 70), Location = new Point(10, y), BackColor = Color.FromArgb(52, 73, 94) };
            item.MouseEnter += (s, e) => item.BackColor = Color.FromArgb(61, 86, 110);
            item.MouseLeave += (s, e) => item.BackColor = Color.FromArgb(52, 73, 94);

            Label nameLabel = new Label { Text = city, Font = new Font("Arial", 16, FontStyle.Bold), ForeColor = Color.White, Location = new Point(15, 20), AutoSize = true };
            item.Controls.Add(nameLabel);

            Label timeLabel = new Label { Text = "00:00:00", Font = new Font("Courier New", 26, FontStyle.Bold), ForeColor = Color.LimeGreen, Location = new Point(140, 15), AutoSize = true };
            item.Controls.Add(timeLabel);

            Label dateLabel = new Label { Text = "Пн, 1 янв 2024", Font = new Font("Arial", 12), ForeColor = Color.LightGray, Location = new Point(310, 25), AutoSize = true };
            item.Controls.Add(dateLabel);

            Button delBtn = new Button { Text = "✕", Size = new Size(30, 30), Location = new Point(item.Width - 45, 20), BackColor = Color.Crimson, ForeColor = Color.White, FlatStyle = FlatStyle.Flat };
            delBtn.Click += (s, e) => RemoveCity(city);
            item.Controls.Add(delBtn);

            clockPanel.Controls.Add(item);
            clockLabels[city] = new Label[] { timeLabel, dateLabel };
            y += 80;
        }
        clockPanel.AutoScrollMinSize = new Size(0, y + 10);
    }

    private void AddCity()
    {
        Form dialog = new Form { Text = "Добавить город", Size = new Size(300, 400), StartPosition = FormStartPosition.CenterParent };
        ListBox listBox = new ListBox { Dock = DockStyle.Fill };
        var available = CitiesMap.Keys.Except(cities).OrderBy(c => c).ToList();
        listBox.DataSource = available;
        Button addBtn = new Button { Text = "Добавить", Dock = DockStyle.Bottom, Height = 40 };
        addBtn.Click += (s, e) => {
            if (listBox.SelectedItem != null)
            {
                cities.Add(listBox.SelectedItem.ToString());
                SaveCities();
                BuildClockList();
                dialog.Close();
            }
        };
        dialog.Controls.Add(listBox);
        dialog.Controls.Add(addBtn);
        dialog.ShowDialog(this);
    }

    private void RemoveCity(string city)
    {
        cities.Remove(city);
        SaveCities();
        BuildClockList();
    }

    private void ClearAll()
    {
        if (cities.Count == 0) return;
        if (MessageBox.Show("Удалить все города?", "Подтверждение", MessageBoxButtons.YesNo) == DialogResult.Yes)
        {
            cities.Clear();
            SaveCities();
            BuildClockList();
        }
    }

    private void StartClock()
    {
        timer = new Timer { Interval = 1000 };
        timer.Tick += (s, e) => UpdateClocks();
        timer.Start();
        UpdateClocks();
    }

    private void UpdateClocks()
    {
        DateTime utc = DateTime.UtcNow;
        foreach (string city in cities)
        {
            if (!CitiesMap.ContainsKey(city) || !clockLabels.ContainsKey(city)) continue;
            double offset = CitiesMap[city];
            DateTime local = utc.AddHours(offset);
            string timeStr;
            if (format24)
                timeStr = local.ToString("HH:mm:ss");
            else
                timeStr = local.ToString("hh:mm:ss tt");
            string dateStr = local.ToString("ddd, d MMM yyyy", new System.Globalization.CultureInfo("ru-RU"));
            var labels = clockLabels[city];
            labels[0].Text = timeStr;
            labels[1].Text = dateStr;
        }
    }

    [STAThread]
    static void Main() { Application.EnableVisualStyles(); Application.Run(new WorldClock()); }
}
