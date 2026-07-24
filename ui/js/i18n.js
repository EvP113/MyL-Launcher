// i18n.js — Internationalization dictionary and translation engine
(function () {
  var translations = {
    ru: {
      // Navigation & Header
      "nav_new_instance": "Новый экземпляр",
      "nav_instances": "Экземпляры",
      "nav_accounts": "Аккаунты",
      "nav_console": "Консоль",
      "nav_settings": "Настройки",
      "offline_account": "Офлайн аккаунт",
      "microsoft_account": "Microsoft Account",
      "no_account": "Нет аккаунта",
      "active": "Активен",

      // Instances Page
      "my_instances": "Мои экземпляры",
      "manage_instances": "Управление сборками Minecraft",
      "total_instances": "Всего экземпляров:",
      "no_instances_found": "Экземпляры не найдены",
      "create_or_import": "Создайте новый экземпляр или импортируйте существующую сборку.",
      "create_instance": "Создать экземпляр",
      "import_zip": "Импортировать ZIP",
      "unassigned_group": "Без группы",

      // Detail Panel
      "ready_to_play": "Готов к запуску",
      "in_game": "В игре",
      "btn_launch": "Запустить",
      "btn_stop": "Закрыть",
      "instance_settings": "Настройки сборки",
      "instance_folder": "Папка экземпляра",
      "btn_delete": "Удалить",
      "info_header": "ИНФОРМАЦИЯ",
      "info_version": "Версия:",
      "info_loader": "Загрузчик:",
      "info_path": "Путь:",

      // Accounts Page
      "account_management": "Управление аккаунтами",
      "account_subtitle": "Авторизация Microsoft и офлайн аккаунты",
      "add_account": "Добавить аккаунт",
      "make_active": "Сделать активным",
      "delete_account": "Удалить аккаунт",
      "no_accounts": "Нет аккаунтов",

      // Console Page
      "game_console": "Консоль игры",
      "console_subtitle": "Логи в реальном времени запущенных процессов Minecraft",
      "autoscroll": "Автоскролл",
      "btn_clear": "Очистить",
      "console_waiting": "Ожидание запуска игры... Логи процесса Minecraft появятся здесь.",

      // Settings Page
      "launcher_settings": "Настройки лаунчера",
      "settings_subtitle": "Кастомизация темы, языка, шрифтов и параметров Java",
      "language_section": "Язык интерфейса (Language)",
      "select_language": "Выберите язык лаунчера",
      "theme": "Тема оформления",
      "select_theme": "Выберите встроенный стиль",
      "font_size": "Размер шрифта",
      "font_size_desc": "Настройка базового кегля для элементов интерфейса",
      "text_density": "Плотность текста (Density)",
      "density_compact": "Компактная",
      "density_normal": "Обычная",
      "density_spacious": "Просторная",
      "launch_behavior": "Поведение и Запуск",
      "action_after_launch": "Действие после запуска игры",
      "action_behavior_desc": "Поведение главного окна лаунчера при старте Minecraft",
      "action_minimize": "Сворачивать в трей",
      "action_close": "Закрывать лаунчер",
      "action_keep": "Оставлять открытым",
      "directories_paths": "Директории и Пути",
      "custom_instances_folder": "Кастомная папка экземпляров",
      "btn_browse": "Обзор...",
      "btn_reset": "Сбросить",
      "default_jvm_args": "Java аргументы по умолчанию",
      "jvm_args_desc": "Универсальные флаги для всех версий",
      "jvm_args_info": "Оптимизированные аргументы JVM. Пользователь может свободно изменять их при необходимости:",
      "ui_animations": "Анимации интерфейса",
      "ui_anim_desc": "Плавные переходы и эффекты наведения",
      "auto_updates": "Автопроверка обновлений",
      "auto_updates_desc": "Регулярно проверять новые версии компонентов",
      "btn_save_settings": "Сохранить настройки",

      // Modals
      "wizard_title": "Создание нового экземпляра",
      "wizard_name": "Имя:",
      "wizard_group": "Группа:",
      "wizard_custom": "Пользовательский",
      "wizard_import": "Импорт",
      "wizard_modrinth": "Modrinth",
      "wizard_version": "Версия",
      "wizard_released": "Выпущено",
      "wizard_type": "Тип",
      "wizard_show": "Показывать",
      "wizard_releases": "Релизы",
      "wizard_snapshots": "Снапшоты",
      "wizard_betas": "Беты",
      "wizard_alphas": "Альфы",
      "wizard_refresh": "Обновить",
      "wizard_modloader": "Загрузчик модов",
      "wizard_none": "Нет",
      "wizard_search": "Поиск...",
      "wizard_btn_create": "Создать",
      "wizard_btn_cancel": "Отмена",

      "settings_modal_title": "Настройки экземпляра",
      "tab_general": "Основные",
      "tab_java": "Java",
      "tab_minecraft": "Minecraft",
      "tab_mods": "Моды",
      "tab_resourcepacks": "Наборы ресурсов",
      "tab_shaders": "Наборы шейдеров",
      "tab_worlds": "Миры",
      "tab_servers": "Серверы",

      "btn_cancel": "Отмена",
      "btn_save": "Сохранить",
      "btn_close": "Закрыть",
      "msa_title": "Авторизация Microsoft",
      "msa_subtitle": "Перейдите по ссылке и введите код:",
      "msa_copy": "Скопировать код",
      "msa_browser": "Открыть браузер",
      "msa_waiting": "Ожидание входа в аккаунт..."
    },
    en: {
      // Navigation & Header
      "nav_new_instance": "New Instance",
      "nav_instances": "Instances",
      "nav_accounts": "Accounts",
      "nav_console": "Console",
      "nav_settings": "Settings",
      "offline_account": "Offline Account",
      "microsoft_account": "Microsoft Account",
      "no_account": "No Account",
      "active": "Active",

      // Instances Page
      "my_instances": "My Instances",
      "manage_instances": "Manage your Minecraft instances",
      "total_instances": "Total instances:",
      "no_instances_found": "No instances found",
      "create_or_import": "Create a new instance or import an existing build.",
      "create_instance": "Create Instance",
      "import_zip": "Import ZIP",
      "unassigned_group": "Unassigned",

      // Detail Panel
      "ready_to_play": "Ready to play",
      "in_game": "In game",
      "btn_launch": "Launch",
      "btn_stop": "Stop",
      "instance_settings": "Instance Settings",
      "instance_folder": "Instance Folder",
      "btn_delete": "Delete",
      "info_header": "INFORMATION",
      "info_version": "Version:",
      "info_loader": "Loader:",
      "info_path": "Path:",

      // Accounts Page
      "account_management": "Account Management",
      "account_subtitle": "Microsoft authentication and offline accounts",
      "add_account": "Add Account",
      "make_active": "Make Active",
      "delete_account": "Delete Account",
      "no_accounts": "No accounts",

      // Console Page
      "game_console": "Game Console",
      "console_subtitle": "Real-time logs of running Minecraft processes",
      "autoscroll": "Autoscroll",
      "btn_clear": "Clear",
      "console_waiting": "Waiting for game launch... Minecraft process logs will appear here.",

      // Settings Page
      "launcher_settings": "Launcher Settings",
      "settings_subtitle": "Theme, language, font, and Java configuration",
      "language_section": "Interface Language",
      "select_language": "Select launcher language",
      "theme": "Theme",
      "select_theme": "Choose built-in style",
      "font_size": "Font Size",
      "font_size_desc": "Adjust root UI typography size",
      "text_density": "Layout Density",
      "density_compact": "Compact",
      "density_normal": "Normal",
      "density_spacious": "Spacious",
      "launch_behavior": "Launch Behavior",
      "action_after_launch": "Action after launching game",
      "action_behavior_desc": "Main launcher window behavior when Minecraft starts",
      "action_minimize": "Minimize to tray",
      "action_close": "Close launcher",
      "action_keep": "Keep open",
      "directories_paths": "Directories & Paths",
      "custom_instances_folder": "Custom Instances Folder",
      "btn_browse": "Browse...",
      "btn_reset": "Reset",
      "default_jvm_args": "Default Java Arguments",
      "jvm_args_desc": "Universal flags for all versions",
      "jvm_args_info": "Optimized JVM arguments. You can customize them as needed:",
      "ui_animations": "Interface Animations",
      "ui_anim_desc": "Smooth transitions and hover effects",
      "auto_updates": "Auto Check Updates",
      "auto_updates_desc": "Check for component updates regularly",
      "btn_save_settings": "Save Settings",

      // Modals
      "wizard_title": "Create New Instance",
      "wizard_name": "Name:",
      "wizard_group": "Group:",
      "wizard_custom": "Custom",
      "wizard_import": "Import",
      "wizard_modrinth": "Modrinth",
      "wizard_version": "Version",
      "wizard_released": "Released",
      "wizard_type": "Type",
      "wizard_show": "Show",
      "wizard_releases": "Releases",
      "wizard_snapshots": "Snapshots",
      "wizard_betas": "Betas",
      "wizard_alphas": "Alphas",
      "wizard_refresh": "Refresh",
      "wizard_modloader": "Modloader",
      "wizard_none": "None",
      "wizard_search": "Search...",
      "wizard_btn_create": "Create",
      "wizard_btn_cancel": "Cancel",

      "settings_modal_title": "Instance Settings",
      "tab_general": "General",
      "tab_java": "Java",
      "tab_minecraft": "Minecraft",
      "tab_mods": "Mods",
      "tab_resourcepacks": "Resource Packs",
      "tab_shaders": "Shader Packs",
      "tab_worlds": "Worlds",
      "tab_servers": "Servers",

      "btn_cancel": "Cancel",
      "btn_save": "Save",
      "btn_close": "Close",
      "msa_title": "Microsoft Authorization",
      "msa_subtitle": "Open the link below and enter code:",
      "msa_copy": "Copy Code",
      "msa_browser": "Open Browser",
      "msa_waiting": "Waiting for login in browser..."
    }
  };

  function getCurrentLang() {
    return localStorage.getItem('myl_lang') || 'ru';
  }

  function t(key, fallback) {
    var lang = getCurrentLang();
    var dict = translations[lang] || translations.ru;
    return dict[key] !== undefined ? dict[key] : (fallback || key);
  }

  function applyLanguage(lang) {
    if (!lang) lang = getCurrentLang();
    localStorage.setItem('myl_lang', lang);
    var dict = translations[lang] || translations.ru;

    document.querySelectorAll('[data-i18n]').forEach(function (el) {
      var key = el.dataset.i18n;
      if (dict[key] !== undefined) {
        el.textContent = dict[key];
      }
    });

    document.querySelectorAll('[data-i18n-placeholder]').forEach(function (el) {
      var key = el.dataset.i18nPlaceholder;
      if (dict[key] !== undefined) {
        el.placeholder = dict[key];
      }
    });
  }

  window.i18n = {
    t: t,
    applyLanguage: applyLanguage,
    getCurrentLang: getCurrentLang,
    translations: translations
  };
})();
