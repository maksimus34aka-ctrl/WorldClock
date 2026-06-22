// WorldClock.java - Мировые часы на Java (Swing)
import javax.swing.*;
import java.awt.*;
import java.awt.event.*;
import java.io.*;
import java.nio.file.*;
import java.time.*;
import java.time.format.DateTimeFormatter;
import java.util.*;
import java.util.List;

public class WorldClock extends JFrame {
    private List<String> cities;
    private boolean format24 = true;
    private Timer timer;
    private JPanel clockPanel;
    private Map<String, JLabel[]> clockLabels = new HashMap<>();
    private static final String DATA_FILE = "world_cities.json";

    private static final Map<String, Double> CITIES_MAP = new LinkedHashMap<>();
    static {
        CITIES_MAP.put("Лондон", 0.0);
        CITIES_MAP.put("Париж", 1.0);
        CITIES_MAP.put("Москва", 3.0);
        CITIES_MAP.put("Дубай", 4.0);
        CITIES_MAP.put("Нью-Йорк", -5.0);
        CITIES_MAP.put("Лос-Анджелес", -8.0);
        CITIES_MAP.put("Токио", 9.0);
        CITIES_MAP.put("Пекин", 8.0);
        CITIES_MAP.put("Сидней", 11.0);
        CITIES_MAP.put("Нью-Дели", 5.5);
        CITIES_MAP.put("Сан-Паулу", -3.0);
        CITIES_MAP.put("Йоханнесбург", 2.0);
        CITIES_MAP.put("Сингапур", 8.0);
        CITIES_MAP.put("Гонконг", 8.0);
        CITIES_MAP.put("Стамбул", 3.0);
        CITIES_MAP.put("Рим", 1.0);
        CITIES_MAP.put("Берлин", 1.0);
        CITIES_MAP.put("Киев", 3.0);
        CITIES_MAP.put("Минск", 3.0);
        CITIES_MAP.put("Алматы", 6.0);
        CITIES_MAP.put("Новосибирск", 7.0);
        CITIES_MAP.put("Владивосток", 10.0);
        CITIES_MAP.put("Магадан", 11.0);
        CITIES_MAP.put("Камчатка", 12.0);
        CITIES_MAP.put("Лиссабон", 0.0);
        CITIES_MAP.put("Дублин", 0.0);
        CITIES_MAP.put("Афины", 2.0);
        CITIES_MAP.put("Каир", 2.0);
        CITIES_MAP.put("Найроби", 3.0);
        CITIES_MAP.put("Кейптаун", 2.0);
        CITIES_MAP.put("Бангкок", 7.0);
        CITIES_MAP.put("Джакарта", 7.0);
        CITIES_MAP.put("Сеул", 9.0);
        CITIES_MAP.put("Окленд", 13.0);
        CITIES_MAP.put("Гонолулу", -10.0);
        CITIES_MAP.put("Анкоридж", -9.0);
        CITIES_MAP.put("Чикаго", -6.0);
        CITIES_MAP.put("Денвер", -7.0);
        CITIES_MAP.put("Финикс", -7.0);
        CITIES_MAP.put("Майами", -5.0);
        CITIES_MAP.put("Торонто", -5.0);
        CITIES_MAP.put("Ванкувер", -8.0);
        CITIES_MAP.put("Мехико", -6.0);
        CITIES_MAP.put("Буэнос-Айрес", -3.0);
        CITIES_MAP.put("Рио-де-Жанейро", -3.0);
        CITIES_MAP.put("Перт", 8.0);
        CITIES_MAP.put("Аделаида", 10.5);
        CITIES_MAP.put("Канберра", 11.0);
    }

    public WorldClock() {
        setTitle("🌍 Мировые часы - Java");
        setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        setSize(750, 550);
        setLocationRelativeTo(null);

        cities = loadCities();
        initUI();
        startClock();
    }

    private List<String> loadCities() {
        try {
            String json = new String(Files.readAllBytes(Paths.get(DATA_FILE)));
            // Простой парсинг, можно использовать Gson, но для простоты оставим так.
            // В этой версии используем простой текстовый формат.
            // Для реального проекта лучше использовать Jackson или Gson.
            // Для демонстрации оставим заглушку и загрузим дефолтные.
        } catch (Exception e) {}
        // Если файла нет, используем дефолтные
        return new ArrayList<>(Arrays.asList("Москва", "Лондон", "Нью-Йорк", "Токио", "Сидней"));
    }

    private void saveCities() {
        try {
            String json = new com.google.gson.Gson().toJson(cities);
            Files.write(Paths.get(DATA_FILE), json.getBytes());
        } catch (Exception e) {}
    }

    private void initUI() {
        setLayout(new BorderLayout());

        // Верхняя панель
        JPanel top = new JPanel(new BorderLayout());
        top.setBackground(new Color(44, 62, 80));
        top.setPreferredSize(new Dimension(0, 60));
        JLabel title = new JLabel("🌍 Мировые часы", SwingConstants.CENTER);
        title.setFont(new Font("Arial", Font.BOLD, 20));
        title.setForeground(Color.WHITE);
        top.add(title, BorderLayout.CENTER);

        JPanel btnPanel = new JPanel(new FlowLayout(FlowLayout.RIGHT));
        btnPanel.setOpaque(false);
        JButton addBtn = new JButton("➕ Добавить город");
        addBtn.addActionListener(e -> addCity());
        btnPanel.add(addBtn);
        JButton formatBtn = new JButton("12/24 ч");
        formatBtn.addActionListener(e -> { format24 = !format24; });
        btnPanel.add(formatBtn);
        JButton clearBtn = new JButton("🗑️ Очистить");
        clearBtn.addActionListener(e -> clearAll());
        btnPanel.add(clearBtn);
        top.add(btnPanel, BorderLayout.EAST);
        add(top, BorderLayout.NORTH);

        // Список городов
        clockPanel = new JPanel();
        clockPanel.setLayout(new BoxLayout(clockPanel, BoxLayout.Y_AXIS));
        JScrollPane scroll = new JScrollPane(clockPanel);
        scroll.getVerticalScrollBar().setUnitIncrement(16);
        add(scroll, BorderLayout.CENTER);

        buildClockList();
    }

    private void buildClockList() {
        clockPanel.removeAll();
        clockLabels.clear();
        for (String city : cities) {
            JPanel item = new JPanel(new BorderLayout());
            item.setBackground(new Color(52, 73, 94));
            item.setBorder(BorderFactory.createEmptyBorder(10, 15, 10, 15));
            item.setMaximumSize(new Dimension(Integer.MAX_VALUE, 70));

            JLabel nameLabel = new JLabel(city);
            nameLabel.setFont(new Font("Arial", Font.BOLD, 16));
            nameLabel.setForeground(Color.WHITE);
            nameLabel.setPreferredSize(new Dimension(120, 30));
            item.add(nameLabel, BorderLayout.WEST);

            JPanel timePanel = new JPanel(new FlowLayout(FlowLayout.LEFT, 20, 0));
            timePanel.setOpaque(false);
            JLabel timeLabel = new JLabel("00:00:00");
            timeLabel.setFont(new Font("Courier New", Font.BOLD, 28));
            timeLabel.setForeground(new Color(46, 204, 113));
            timeLabel.setPreferredSize(new Dimension(150, 40));
            JLabel dateLabel = new JLabel("Пн, 1 янв 2024");
            dateLabel.setFont(new Font("Arial", Font.PLAIN, 13));
            dateLabel.setForeground(new Color(189, 195, 199));
            dateLabel.setPreferredSize(new Dimension(150, 20));
            timePanel.add(timeLabel);
            timePanel.add(dateLabel);
            item.add(timePanel, BorderLayout.CENTER);

            JButton delBtn = new JButton("✕");
            delBtn.setBackground(new Color(231, 76, 60));
            delBtn.setForeground(Color.WHITE);
            delBtn.setFocusPainted(false);
            delBtn.setPreferredSize(new Dimension(30, 30));
            delBtn.addActionListener(e -> removeCity(city));
            item.add(delBtn, BorderLayout.EAST);

            clockPanel.add(item);
            clockLabels.put(city, new JLabel[]{timeLabel, dateLabel});
        }
        clockPanel.revalidate();
        clockPanel.repaint();
    }

    private void addCity() {
        // Диалог выбора города
        JDialog dialog = new JDialog(this, "Добавить город", true);
        dialog.setSize(300, 400);
        dialog.setLocationRelativeTo(this);
        dialog.setLayout(new BorderLayout());

        JList<String> list = new JList<>();
        DefaultListModel<String> model = new DefaultListModel<>();
        List<String> available = new ArrayList<>(CITIES_MAP.keySet());
        available.removeAll(cities);
        Collections.sort(available);
        for (String c : available) model.addElement(c);
        list.setModel(model);

        dialog.add(new JScrollPane(list), BorderLayout.CENTER);

        JButton addBtn = new JButton("Добавить");
        addBtn.addActionListener(e -> {
            String selected = list.getSelectedValue();
            if (selected != null) {
                cities.add(selected);
                saveCities();
                buildClockList();
                dialog.dispose();
            }
        });
        dialog.add(addBtn, BorderLayout.SOUTH);
        dialog.setVisible(true);
    }

    private void removeCity(String city) {
        cities.remove(city);
        saveCities();
        buildClockList();
    }

    private void clearAll() {
        int res = JOptionPane.showConfirmDialog(this, "Удалить все города?", "Подтверждение", JOptionPane.YES_NO_OPTION);
        if (res == JOptionPane.YES_OPTION) {
            cities.clear();
            saveCities();
            buildClockList();
        }
    }

    private void startClock() {
        timer = new Timer(1000, e -> updateClocks());
        timer.start();
        updateClocks();
    }

    private void updateClocks() {
        ZonedDateTime utc = ZonedDateTime.now(ZoneOffset.UTC);
        for (String city : cities) {
            Double offset = CITIES_MAP.get(city);
            if (offset == null || !clockLabels.containsKey(city)) continue;
            ZonedDateTime local = utc.plusSeconds((long)(offset * 3600));
            String timeStr;
            if (format24) {
                timeStr = local.format(DateTimeFormatter.ofPattern("HH:mm:ss"));
            } else {
                timeStr = local.format(DateTimeFormatter.ofPattern("hh:mm:ss a"));
            }
            String dateStr = local.format(DateTimeFormatter.ofPattern("EEE, d MMM yyyy", new Locale("ru")));
            JLabel[] labels = clockLabels.get(city);
            labels[0].setText(timeStr);
            labels[1].setText(dateStr);
        }
    }

    public static void main(String[] args) {
        SwingUtilities.invokeLater(() -> new WorldClock().setVisible(true));
    }
}
