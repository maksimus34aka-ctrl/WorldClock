<?php
// world_clock.php - Мировые часы на PHP (веб-сервер)
session_start();

$cities_map = [
    "Лондон" => 0, "Париж" => 1, "Москва" => 3, "Дубай" => 4,
    "Нью-Йорк" => -5, "Лос-Анджелес" => -8, "Токио" => 9,
    "Пекин" => 8, "Сидней" => 11, "Нью-Дели" => 5.5,
    "Сан-Паулу" => -3, "Йоханнесбург" => 2, "Сингапур" => 8,
    "Гонконг" => 8, "Стамбул" => 3, "Рим" => 1, "Берлин" => 1,
    "Киев" => 3, "Минск" => 3, "Алматы" => 6, "Новосибирск" => 7,
    "Владивосток" => 10, "Магадан" => 11, "Камчатка" => 12,
    "Лиссабон" => 0, "Дублин" => 0, "Афины" => 2, "Каир" => 2,
    "Найроби" => 3, "Кейптаун" => 2, "Бангкок" => 7, "Джакарта" => 7,
    "Сеул" => 9, "Окленд" => 13, "Гонолулу" => -10, "Анкоридж" => -9,
    "Чикаго" => -6, "Денвер" => -7, "Финикс" => -7, "Майами" => -5,
    "Торонто" => -5, "Ванкувер" => -8, "Мехико" => -6, "Буэнос-Айрес" => -3,
    "Рио-де-Жанейро" => -3, "Перт" => 8, "Аделаида" => 10.5, "Канберра" => 11
];

if (!isset($_SESSION['cities'])) {
    $_SESSION['cities'] = ["Москва", "Лондон", "Нью-Йорк", "Токио", "Сидней"];
}
if (!isset($_SESSION['format24'])) {
    $_SESSION['format24'] = true;
}

$cities = $_SESSION['cities'];
$format24 = $_SESSION['format24'];

// Обработка AJAX запросов
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    $action = $_POST['action'] ?? '';
    if ($action === 'add') {
        $city = $_POST['city'] ?? '';
        if ($city && !in_array($city, $cities) && isset($cities_map[$city])) {
            $cities[] = $city;
            $_SESSION['cities'] = $cities;
        }
        echo json_encode(['success' => true]);
        exit;
    } elseif ($action === 'remove') {
        $city = $_POST['city'] ?? '';
        $cities = array_filter($cities, fn($c) => $c !== $city);
        $_SESSION['cities'] = array_values($cities);
        echo json_encode(['success' => true]);
        exit;
    } elseif ($action === 'clear') {
        $_SESSION['cities'] = [];
        echo json_encode(['success' => true]);
        exit;
    } elseif ($action === 'toggle_format') {
        $_SESSION['format24'] = !$_SESSION['format24'];
        echo json_encode(['success' => true]);
        exit;
    } elseif ($action === 'get_data') {
        echo json_encode(['cities' => $cities, 'format24' => $_SESSION['format24']]);
        exit;
    }
}
?>
<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🌍 Мировые часы - PHP</title>
    <style>
        * { box-sizing: border-box; margin: 0; }
        body { background: #1e2a3a; font-family: 'Segoe UI', system-ui; display: flex; justify-content: center; min-height: 100vh; padding: 20px; }
        .container { max-width: 800px; width: 100%; background: #2c3e50; border-radius: 16px; padding: 20px; box-shadow: 0 10px 30px rgba(0,0,0,0.3); }
        .header { display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 10px; margin-bottom: 20px; color: white; }
        .header h1 { font-size: 24px; }
        .controls { display: flex; gap: 8px; flex-wrap: wrap; }
        .controls button, .controls select { padding: 6px 12px; border: none; border-radius: 6px; cursor: pointer; background: #3498db; color: white; }
        .controls button:hover { background: #2980b9; }
        .controls .danger { background: #e74c3c; }
        .controls .danger:hover { background: #c0392b; }
        .clock-list { display: flex; flex-direction: column; gap: 8px; max-height: 600px; overflow-y: auto; padding-right: 5px; }
        .clock-item { background: #34495e; border-radius: 12px; padding: 12px 16px; display: flex; align-items: center; flex-wrap: wrap; gap: 12px; color: white; }
        .clock-item:hover { background: #3d566e; }
        .city-name { font-size: 16px; font-weight: 600; min-width: 120px; }
        .time-display { font-size: 28px; font-family: 'Courier New', monospace; font-weight: bold; min-width: 130px; color: #2ecc71; }
        .date-display { font-size: 13px; color: #bdc3c7; min-width: 140px; }
        .delete-btn { margin-left: auto; background: #e74c3c; border: none; color: white; width: 28px; height: 28px; border-radius: 50%; cursor: pointer; font-size: 16px; line-height: 1; }
        .delete-btn:hover { background: #c0392b; }
        .empty-msg { color: #7f8c8d; text-align: center; padding: 40px; font-size: 18px; }
        ::-webkit-scrollbar { width: 6px; }
        ::-webkit-scrollbar-track { background: #2c3e50; border-radius: 8px; }
        ::-webkit-scrollbar-thumb { background: #5d6d7e; border-radius: 8px; }
        @media (max-width: 600px) { .time-display { font-size: 22px; min-width: 100px; } .city-name { min-width: 80px; font-size: 14px; } }
    </style>
</head>
<body>
<div class="container">
    <div class="header">
        <h1>🌍 Мировые часы</h1>
        <div class="controls">
            <select id="citySelect"><option value="">-- Добавить город --</option></select>
            <button id="addBtn">➕ Добавить</button>
            <button id="formatBtn">12/24 ч</button>
            <button id="clearBtn" class="danger">🗑️ Очистить</button>
        </div>
    </div>
    <div class="clock-list" id="clockList"></div>
</div>
<script>
    const citiesMap = <?= json_encode($cities_map) ?>;
    let cities = <?= json_encode($cities) ?>;
    let format24 = <?= json_encode($format24) ?>;

    function saveToServer(action, data = {}) {
        const formData = new URLSearchParams({ action, ...data });
        return fetch(window.location.href, { method: 'POST', headers: { 'Content-Type': 'application/x-www-form-urlencoded' }, body: formData });
    }

    function updateClock() {
        const now = new Date();
        const utc = now.getTime() + now.getTimezoneOffset() * 60000;
        const list = document.getElementById('clockList');
        list.innerHTML = '';
        if (cities.length === 0) {
            list.innerHTML = '<div class="empty-msg">🌍 Нет городов. Добавьте первый!</div>';
            return;
        }
        cities.forEach(city => {
            const offset = citiesMap[city];
            if (offset === undefined) return;
            const localTime = new Date(utc + offset * 3600000);
            let hours = localTime.getHours();
            const minutes = String(localTime.getMinutes()).padStart(2, '0');
            const seconds = String(localTime.getSeconds()).padStart(2, '0');
            let timeStr;
            if (format24) {
                timeStr = `${String(hours).padStart(2, '0')}:${minutes}:${seconds}`;
            } else {
                const ampm = hours >= 12 ? 'PM' : 'AM';
                hours = hours % 12 || 12;
                timeStr = `${String(hours).padStart(2, '0')}:${minutes}:${seconds} ${ampm}`;
            }
            const days = ['Вс', 'Пн', 'Вт', 'Ср', 'Чт', 'Пт', 'Сб'];
            const months = ['янв', 'фев', 'мар', 'апр', 'мая', 'июн', 'июл', 'авг', 'сен', 'окт', 'ноя', 'дек'];
            const dateStr = `${days[localTime.getDay()]}, ${localTime.getDate()} ${months[localTime.getMonth()]} ${localTime.getFullYear()}`;

            const item = document.createElement('div');
            item.className = 'clock-item';
            item.innerHTML = `
                <span class="city-name">${city}</span>
                <span class="time-display">${timeStr}</span>
                <span class="date-display">${dateStr}</span>
                <button class="delete-btn" data-city="${city}">✕</button>
            `;
            list.appendChild(item);
        });
        document.querySelectorAll('.delete-btn').forEach(btn => {
            btn.addEventListener('click', function() {
                const city = this.dataset.city;
                saveToServer('remove', { city }).then(() => {
                    cities = cities.filter(c => c !== city);
                    updateClock();
                    updateSelect();
                });
            });
        });
    }

    function updateSelect() {
        const sel = document.getElementById('citySelect');
        const current = sel.value;
        sel.innerHTML = '<option value="">-- Добавить город --</option>';
        const available = Object.keys(citiesMap).filter(c => !cities.includes(c));
        available.sort();
        available.forEach(c => {
            const opt = document.createElement('option');
            opt.value = c;
            opt.textContent = c;
            sel.appendChild(opt);
        });
        if (current && available.includes(current)) sel.value = current;
    }

    document.getElementById('addBtn').addEventListener('click', () => {
        const sel = document.getElementById('citySelect');
        const city = sel.value;
        if (!city) return;
        if (!cities.includes(city)) {
            saveToServer('add', { city }).then(() => {
                cities.push(city);
                updateClock();
                updateSelect();
            });
        }
        sel.value = '';
    });

    document.getElementById('formatBtn').addEventListener('click', () => {
        saveToServer('toggle_format').then(() => {
            format24 = !format24;
            updateClock();
        });
    });

    document.getElementById('clearBtn').addEventListener('click', () => {
        if (cities.length === 0) return;
        if (confirm('Удалить все города?')) {
            saveToServer('clear').then(() => {
                cities = [];
                updateClock();
                updateSelect();
            });
        }
    });

    // Инициализация и автообновление
    updateSelect();
    updateClock();
    setInterval(updateClock, 1000);
</script>
</body>
</html>
