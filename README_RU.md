[![GitHub Release](https://img.shields.io/github/v/release/AndrewShedov/enter-text--LARS?style=for-the-badge&logo=github&logoColor=white&labelColor=black&color=%23f3f3f3)](https://github.com/AndrewShedov/enter-text--LARS/releases)&nbsp;[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=5865F2&logoColor=black&labelColor=black&color=%23f3f3f3)](https://github.com/AndrewShedov/enter-text--LARS/blob/main/LICENSE)&nbsp;[![Members](https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=&logo=discord&logoColor=white&labelColor=black&color=%23f3f3f3&query=$.approximate_member_count&url=https%3A%2F%2Fdiscord.com%2Fapi%2Finvites%2FENB7RbxVZE%3Fwith_counts%3Dtrue)](https://discord.gg/ENB7RbxVZE)

# Enter Text (LARS)

[![En](https://img.shields.io/badge/En-black?style=flat&logoColor=whit)](https://github.com/AndrewShedov/enter-text--LARS)<br>

> **Исследовательская статья:** <br>
>  [💎 Дневник разработчика CRYSTAL #3: Сравнительное исследование производительности и кибербезопасности клиент-серверного взаимодействия в технологических стеках LARS и SAHAR](https://shedov.top/ru/dnevnik-razrabotchika-crystal-3-sravnitelnoe-issledovanie-lars-i-sahar/)

Это приложение является прототипом, разработанным в рамках поиска нового технологического стека для [CRYSTAL v3.0](https://github.com/CrystalSystems).<br>
Данный прототип позволяет **добавлять**, **отображать**, **обновлять** и **удалять текст**, демонстрируя полный цикл **CRUD**-операций с базой данных ScyllaDB.<br>
**Блокирующий рендеринг на стороне сервера (Blocking SSR)** полностью рендерит содержимое базы данных непосредственно на сервере, гарантируя **отличную индексацию поисковыми системами** и высочайшую **SEO-оптимизацию**.

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--LARS/refs/heads/main/assets/gif2.gif"/>
</p>

**Содержание:**<br>
1. [Структура проекта и характеристики локального ПК](#paragraph_1)
2. [Ключевые особенности](#paragraph_2)
3. [Установка и настройка](#paragraph_3)
4. [Подготовка базы данных](#paragraph_4)
5. [Запуск проекта](#paragraph_5)

<span id="paragraph_1"></span> 
### 1. Структура проекта и характеристики локального ПК 

**Строение:** <br>
[Full code](https://github.com/AndrewShedov/enter-text--LARS/tree/main/main) | [Cargo.toml](https://github.com/AndrewShedov/enter-text--LARS/blob/main/main/Cargo.toml)<br>

**Структура:** <br>
**L**eptos v0.8.2.<br>
**A**ctix Web v4.15.0.<br>
**R**ust v1.98.0.<br>
**S**cyllaDB v2026.2.6 (драйвер: v1.8.0).<br>

**Характеристики локального ПК:** <br>
ОС: Debian 12.<br>

<span id="paragraph_2"></span> 
### 2. Ключевые особенности: 

**2.1. Авто-схема (Auto-Schema), однострочная архитектура и константный id**<br>
При запуске приложения автоматически создается таблица <code>data</code> (если она не существует) для хранения введенного текста. Во время операции сохранения в таблице формируется строка, состоящая из трех столбцов: <code>id</code> (Первичный ключ), <code>content</code> (текстовые данные) и <code>created_at</code> (временная метка). Для введенного текста используется постоянный <code>id</code> в формате <code>UUID (11111111-1111-1111-1111-111111111111)</code>. Вместо создания множества записей система использует операцию <code>INSERT</code> как <code>"upsert"</code> (обновление существующей записи). Поскольку <code>id</code> всегда один и тот же, любая операция сохранения просто перезаписывает данные в столбце content для этой конкретной строки:

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--LARS/refs/heads/main/assets/screenshot_1.png"/>
</p>
<p align="center"><strong>Скриншот 1: Отображение единственной строки в БД</strong></p>


**2.2. Блокирующий SSR (SsrMode::PartiallyBlocked)**<br>
Режим блокирующего рендеринга на стороне сервера. Это гарантирует, что динамический контент из ScyllaDB (и весь остальной текст сайта) "внедряется" в структуру HTML непосредственно на сервере. В результате поисковые роботы получают полностью отрендеренный документ, что обеспечивает 100% индексацию и высокие показатели SEO:

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--LARS/refs/heads/main/assets/screenshot_2.png"/>
</p>
<p align="center"><strong>Скриншот 2: Рендеринг на стороне сервера (SSR). Просмотр исходного кода в браузере (Ctrl+U)</strong></p>


**2.3. Точечная реактивность (Fine-grained Reactivity)**<br>
При обновлении текста перерисовывается только конкретный узел DOM, содержащий этот текст, в то время как остальная часть страницы остается нетронутой. Такое поведение достигается благодаря реактивным сигналам Leptos.

**2.4. Работает с отключенным JavaScript**<br>
Приложение спроектировано таким образом, чтобы оставаться полностью функциональным даже в случае отключения JavaScript в браузере пользователя. Опираясь на серверный рендеринг и компонент `<ActionForm>` от Leptos, базовые операции с данными (добавление и удаление контента) плавно переходят на стандартную отправку HTML-форм с классической перезагрузкой страницы. Это обеспечивает абсолютную отказоустойчивость.

Вы можете протестировать это поведение, отключив JavaScript в своем браузере:
* **Google Chrome / Chromium:** Откройте Инструменты разработчика (`F12`), нажмите `Ctrl+Shift+P` (или `Cmd+Shift+P` на macOS) для открытия командного меню, введите `Disable JavaScript` и нажмите Enter.
* **Mozilla Firefox:** Введите `about:config` в адресной строке, примите риски, найдите параметр `javascript.enabled` и дважды кликните по нему, чтобы переключить значение на `false`.

**2.5. Изоморфный доступ к данным и реактивный UI**<br>
Использование <code>Resource::new_blocking</code> обеспечивает бесшовную синхронизацию состояния между сервером и клиентом. Оно автоматически отслеживает изменения в базе данных через версии действий (action versions), позволяя пользовательскому интерфейсу мгновенно переключать кнопки (например, переключаться между "Добавить" и "Обновить" или показывать кнопку "Удалить") без перезагрузки страницы.

**2.6. Асинхронная интеграция ScyllaDB**<br>
Высокопроизводительное асинхронное подключение через <code>scylla-rust-driver</code>. Благодаря использованию разделяемого состояния <code>Arc&lt;Session&gt;</code>, сервер эффективно распределяет ресурсы и может обрабатывать тысячи одновременных запросов без блокировки потоков процессора. Пока база данных подготавливает ответ, процессор остается свободным для других задач, обеспечивая максимальную отзывчивость системы при экстремальных нагрузках.

**2.7. Изоляция SSR**<br>
Весь код взаимодействия с базой данных защищен макросами <code>#[cfg(feature = "ssr")]</code>. Это гарантирует, что драйверы базы данных и чувствительная логика никогда не покинут сервер.

**2.8. Информативное логирование сервера**<br>
Система выводит в консоль подробные отчеты об операциях:

<p align="center">
    <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--LARS/refs/heads/main/assets/screenshot_3.2.webp" width="750" />
</p>
<p align="center"><strong>Скриншот 3: Лог готовности ScyllaDB</strong></p>

<span id="paragraph_3"></span> 
### 3. Установка и настройка

**Примечание о совместимости:** Работа данного проекта проверена на Debian 12. Разработка на Windows не рекомендуется, так как процесс установки Leptos и ScyllaDB на этой системе может вызывать критические ошибки. Для корректной работы требуется Linux-среда.

**3.1. Подготовка окружения (Debian 12 и аналоги)**<br>

Установка системных зависимостей:

```bash
sudo apt update && sudo apt install build-essential pkg-config libssl-dev -y
```

**3.2. Установка Rust и инструментов Leptos**

**3.2.1. Установите Rust (устанавливает текущую стабильную версию):**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**3.2.2. Примените переменные окружения:**

```bash
source $HOME/.cargo/env
```

**3.2.3. Добавьте поддержку WebAssembly:**

```bash
rustup target add wasm32-unknown-unknown
```

**3.2.4. Установите инструмент сборки cargo-leptos:**

```bash
cargo install --locked cargo-leptos
```

**3.3 Установка и настройка ScyllaDB.**

Установка производится непосредственно в систему Debian 12 с использованием официального репозитория ScyllaDB.

**3.3.1. Обновите индексы пакетов:**

```bash
sudo apt-get update
```

**3.3.2. Установите необходимые системные утилиты:**

```bash
sudo apt-get install -y apt-transport-https curl gnupg
```

**3.3.3. Добавьте официальный репозиторий ScyllaDB в систему:**

📌 Примечание: По умолчанию этот скрипт автоматически регистрирует репозиторий для последней стабильной версии ScyllaDB Open Source. Чтобы явно установить ScyllaDB v2026.2.6, используемую в этом прототипе, выполните команду с флагом версии:

```bash
curl -sSf https://get.scylladb.com/server | sudo bash -s -- --scylla-version 2026.2
```


**3.3.4. Интерактивная конфигурация и установка ScyllaDB:**

```bash
sudo scylla_setup
```

**3.3.5. Запустите службу сервера ScyllaDB:**

```bash
sudo systemctl start scylla-server
```

**3.3.6. Проверьте статус узлов кластера:**

```bash
nodetool status
```

<span id="paragraph_4"></span> 
### 4. Подготовка базы данных

Перед первым запуском проекта необходимо создать пространство ключей (Keyspace) в ScyllaDB.

Войдите в консоль базы данных с помощью команды:

```bash
cqlsh
```

и выполните запрос:

```bash
CREATE KEYSPACE IF NOT EXISTS prototype 
WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};
```

<span id="paragraph_5"></span> 
### 5. Запуск проекта

**5.1. Клонирование репозитория проекта:**

```bash
git clone https://github.com/AndrewShedov/enter-text--LARS && cd enter-text--LARS/main 
```

**5.2. Запуск проекта:**

```bash
cargo leptos watch
```

После завершения сборки приложение будет доступно по адресу:

<code>http://127.0.0.1:3000</code>

Таблица **data** внутри пространства ключей **prototype** будет создана автоматически при первом запросе приложения к базе данных благодаря встроенной логике Авто-схемы (Auto-Schema).<br>

По умолчанию ScyllaDB настроена на работу с адресом <code>127.0.0.1:9042</code>.<br>
Вы можете проверить адрес, введя команду в терминал:

```bash
cqlsh
```

После ввода команды должен отобразиться адрес:<br>

<code>Connected to at 127.0.0.1:9042</code>
<br>
<br>
>Lars — это традиционное скандинавское мужское имя: https://en.wikipedia.org/wiki/Lars, и именно оно вдохновило меня при выборе названия стека.<br> 
Я также очень люблю Скандинавию и скандинавскую мифологию.<br>
Я добавляю это, потому что многим было интересно, как я придумал название 😅
<br>
<br>

[![SHEDOV.TOP](https://img.shields.io/badge/SHEDOV.TOP-black?style=for-the-badge)](https://shedov.top/) [![CRYSTAL](https://img.shields.io/badge/CRYSTAL-black?style=for-the-badge)](https://crystal.you/AndrewShedov) [![Discord](https://img.shields.io/badge/Discord-black?style=for-the-badge&logo=discord&color=black&logoColor=white)](https://discord.gg/ENB7RbxVZE) [![Telegram](https://img.shields.io/badge/Telegram-black?style=for-the-badge&logo=telegram&color=black&logoColor=white)](https://t.me/ShedovTop) [![X](https://img.shields.io/badge/%20-black?style=for-the-badge&logo=x&logoColor=white)](https://x.com/AndrewShedov) [![YouTube](https://img.shields.io/badge/YouTube-black?style=for-the-badge&logo=youtube)](https://www.youtube.com/@AndrewShedov) [![VK](https://img.shields.io/badge/VK-black?style=for-the-badge&logo=vk)](https://vk.com/ShedovTop) [![VK Video](https://img.shields.io/badge/VK%20Video-black?style=for-the-badge&logo=vk)](https://vkvideo.ru/@ShedovTop)