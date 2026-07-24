// i18n.js — Internationalization dictionary and translation engine
(function () {
  var translations = {
    ru: {
      // Navigation & Header
      "nav_new_instance": "\u041d\u043e\u0432\u044b\u0439 \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440",
      "nav_instances": "\u042d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u044b",
      "nav_accounts": "\u0410\u043a\u043a\u0430\u0443\u043d\u0442\u044b",
      "nav_console": "\u041a\u043e\u043d\u0441\u043e\u043b\u044c",
      "nav_settings": "\u041d\u0430\u0441\u0442\u0440\u043e\u0439\u043a\u0438",
      "offline_account": "\u041e\u0444\u043b\u0430\u0439\u043d \u0430\u043a\u043a\u0430\u0443\u043d\u0442",
      "microsoft_account": "Microsoft Account",
      "no_account": "\u041d\u0435\u0442 \u0430\u043a\u043a\u0430\u0443\u043d\u0442\u0430",
      "active": "\u0410\u043a\u0442\u0438\u0432\u0435\u043d",

      // Languages
      "lang_ru": "\u0420\u0443\u0441\u0441\u043a\u0438\u0439 (Russian)",
      "lang_en": "English (\u0410\u043d\u0433\u043b\u0438\u0439\u0441\u043a\u0438\u0439)",

      // Themes
      "theme_dark_title": "Prism Dark (\u0414\u0435\u0444\u043e\u043b\u0442)",
      "theme_dark_desc": "\u0422\u0451\u043c\u043d\u0430\u044f \u0442\u0435\u043c\u0430 \u0441 \u0444\u0438\u043e\u043b\u0435\u0442\u043e\u0432\u044b\u043c \u0430\u043a\u0446\u0435\u043d\u0442\u043e\u043c",
      "theme_onyx_title": "Midnight Onyx",
      "theme_onyx_desc": "\u0413\u043b\u0443\u0431\u043e\u043a\u0438\u0439 \u0447\u0451\u0440\u043d\u044b\u0439 \u0446\u0432\u0435\u0442",
      "theme_light_title": "Cloud White",
      "theme_light_desc": "\u0421\u0432\u0435\u0442\u043b\u0430\u044f \u043b\u0430\u043a\u043e\u043d\u0438\u0447\u043d\u0430\u044f \u0442\u0435\u043c\u0430",
      "theme_ember_title": "Ember Flow",
      "theme_ember_desc": "\u0422\u0451\u043f\u043b\u044b\u0435 \u043e\u0440\u0430\u043d\u0436\u0435\u0432\u044b\u0435 \u0442\u043e\u043d\u0430",

      // Instances Page
      "my_instances": "\u041c\u043e\u0438 \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u044b",
      "manage_instances": "\u0423\u043f\u0440\u0430\u0432\u043b\u0435\u043d\u0438\u0435 \u0441\u0431\u043e\u0440\u043a\u0430\u043c\u0438 Minecraft",
      "total_instances": "\u0412\u0441\u0435\u0433\u043e \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u043e\u0432:",
      "no_instances_found": "\u042d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u044b \u043d\u0435 \u043d\u0430\u0439\u0434\u0435\u043d\u044b",
      "create_or_import": "\u0421\u043e\u0437\u0434\u0430\u0439\u0442\u0435 \u043d\u043e\u0432\u044b\u0439 \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440 \u0438\u043b\u0438 \u0438\u043c\u043f\u043e\u0440\u0442\u0438\u0440\u0443\u0439\u0442\u0435 \u0441\u0443\u0449\u0435\u0441\u0442\u0432\u0443\u044e\u0449\u0443\u044e \u0441\u0431\u043e\u0440\u043a\u0443.",
      "create_instance": "\u0421\u043e\u0437\u0434\u0430\u0442\u044c \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440",
      "import_zip": "\u0418\u043c\u043f\u043e\u0440\u0442\u0438\u0440\u043e\u0432\u0430\u0442\u044c ZIP",
      "unassigned_group": "\u0411\u0435\u0437 \u0433\u0440\u0443\u043f\u043f\u044b",
      "no_instances_title": "\u042d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u044b \u043d\u0435 \u043d\u0430\u0439\u0434\u0435\u043d\u044b",
      "no_instances_desc": "\u0421\u043e\u0437\u0434\u0430\u0439\u0442\u0435 \u043d\u043e\u0432\u044b\u0439 \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440 \u0438\u043b\u0438 \u0438\u043c\u043f\u043e\u0440\u0442\u0438\u0440\u0443\u0439\u0442\u0435 \u0441\u0443\u0449\u0435\u0441\u0442\u0432\u0443\u044e\u0449\u0443\u044e \u0441\u0431\u043e\u0440\u043a\u0443.",
      "btn_create_first": "\u0421\u043e\u0437\u0434\u0430\u0442\u044c \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440",
      "btn_import_first": "\u0418\u043c\u043f\u043e\u0440\u0442\u0438\u0440\u043e\u0432\u0430\u0442\u044c ZIP",

      // Detail Panel
      "ready_to_play": "\u0413\u043e\u0442\u043e\u0432 \u043a \u0437\u0430\u043f\u0443\u0441\u043a\u0443",
      "in_game": "\u0412 \u0438\u0433\u0440\u0435",
      "btn_launch": "\u0417\u0430\u043f\u0443\u0441\u0442\u0438\u0442\u044c",
      "btn_stop": "\u0417\u0430\u043a\u0440\u044b\u0442\u044c",
      "instance_settings": "\u041d\u0430\u0441\u0442\u0440\u043e\u0439\u043a\u0438 \u0441\u0431\u043e\u0440\u043a\u0438",
      "instance_folder": "\u041f\u0430\u043f\u043a\u0430 \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u0430",
      "btn_delete": "\u0423\u0434\u0430\u043b\u0438\u0442\u044c",
      "info_header": "\u0418\u041d\u0424\u041e\u0420\u041c\u0410\u0426\u0418\u042f",
      "info_version": "\u0412\u0435\u0440\u0441\u0438\u044f:",
      "info_loader": "\u0417\u0430\u0433\u0440\u0443\u0437\u0447\u0438\u043a:",
      "info_path": "\u041f\u0443\u0442\u044c:",

      // Accounts Page
      "account_management": "\u0423\u043f\u0440\u0430\u0432\u043b\u0435\u043d\u0438\u0435 \u0430\u043a\u043a\u0430\u0443\u043d\u0442\u0430\u043c\u0438",
      "account_subtitle": "\u0410\u0432\u0442\u043e\u0440\u0438\u0437\u0430\u0446\u0438\u044f Microsoft \u0438 \u043e\u0444\u043b\u0430\u0439\u043d \u0430\u043a\u043a\u0430\u0443\u043d\u0442\u044b",
      "add_account": "\u0414\u043e\u0431\u0430\u0432\u0438\u0442\u044c \u0430\u043a\u043a\u0430\u0443\u043d\u0442",
      "make_active": "\u0421\u0434\u0435\u043b\u0430\u0442\u044c \u0430\u043a\u0442\u0438\u0432\u043d\u044b\u043c",
      "delete_account": "\u0423\u0434\u0430\u043b\u0438\u0442\u044c \u0430\u043a\u043a\u0430\u0443\u043d\u0442",
      "no_accounts": "\u041d\u0435\u0442 \u0430\u043a\u043a\u0430\u0443\u043d\u0442\u043e\u0432",
      "add_account_placeholder": "\u0414\u043e\u0431\u0430\u0432\u0438\u0442\u044c \u0430\u043a\u043a\u0430\u0443\u043d\u0442",

      // Console Page
      "game_console": "\u041a\u043e\u043d\u0441\u043e\u043b\u044c \u0438\u0433\u0440\u044b",
      "console_subtitle": "\u041b\u043e\u0433\u0438 \u0432 \u0440\u0435\u0430\u043b\u044c\u043d\u043e\u043c \u0432\u0440\u0435\u043c\u0435\u043d\u0438 \u0437\u0430\u043f\u0443\u0449\u0435\u043d\u043d\u044b\u0445 \u043f\u0440\u043e\u0446\u0435\u0441\u0441\u043e\u0432 Minecraft",
      "autoscroll": "\u0410\u0432\u0442\u043e\u0441\u043a\u0440\u043e\u043b\u043b",
      "btn_clear": "\u041e\u0447\u0438\u0441\u0442\u0438\u0442\u044c",
      "console_waiting": "\u041e\u0436\u0438\u0434\u0430\u043d\u0438\u0435 \u0437\u0430\u043f\u0443\u0441\u043a\u0430 \u0438\u0433\u0440\u044b... \u041b\u043e\u0433\u0438 \u043f\u0440\u043e\u0446\u0435\u0441\u0441\u0430 Minecraft \u043f\u043e\u044f\u0422\u0438\u0442\u0441\u044f \u0437\u0434\u0435\u0441\u044c.",

      // Settings Page
      "launcher_settings": "\u041d\u0430\u0441\u0442\u0440\u043e\u0439\u043a\u0438 \u043b\u0430\u0443\u043d\u0447\u0435\u0440\u0430",
      "settings_subtitle": "\u041a\u0430\u0441\u0442\u043e\u043c\u0438\u0437\u0430\u0446\u0438\u044f \u0442\u0435\u043c\u044b, \u044f\u0437\u044b\u043a\u0430, \u0448\u0440\u0438\u0444\u0442\u043e\u0432 \u0438 \u043f\u0430\u0440\u0430\u043c\u0435\u0442\u0440\u043e\u0432 Java",
      "language_section": "\u042f\u0437\u044b\u043a \u0438\u043d\u0442\u0435\u0440\u0444\u0435\u0439\u0441\u0430 (Language)",
      "select_language": "\u0412\u044b\u0431\u0435\u0440\u0438\u0442\u0435 \u044f\u0437\u044b\u043a \u043b\u0430\u0443\u043d\u0447\u0435\u0440\u0430",
      "theme": "\u0422\u0435\u043c\u0430 \u043e\u0444\u043e\u0440\u043c\u043b\u0435\u043d\u0438\u044f",
      "select_theme": "\u0412\u044b\u0431\u0435\u0440\u0438\u0442\u0435 \u0432\u0441\u0442\u0440\u043e\u0435\u043d\u043d\u044b\u0439 \u0441\u0442\u0438\u043b\u044c",
      "font_size": "\u0420\u0430\u0437\u043c\u0435\u0440 \u0448\u0440\u0438\u0444\u0442\u0430",
      "font_size_desc": "\u041d\u0430\u0441\u0442\u0440\u043e\u0439\u043a\u0430 \u0431\u0430\u0437\u043e\u0432\u043e\u0433\u043e \u043a\u0435\u0433\u043b\u044f \u0434\u043b\u044f \u044d\u043b\u0435\u043c\u0435\u043d\u0442\u043e\u0432 \u0438\u043d\u0442\u0435\u0440\u0444\u0435\u0439\u0441\u0430",
      "text_density": "\u041f\u043b\u043e\u0442\u043d\u043e\u0441\u0442\u044c \u0442\u0435\u043a\u0441\u0442\u0430 (Density)",
      "density_compact": "\u041a\u043e\u043c\u043f\u0430\u043a\u0442\u043d\u0430\u044f",
      "density_normal": "\u041e\u0431\u044b\u0447\u043d\u0430\u044f",
      "density_spacious": "\u041f\u0440\u043e\u0441\u0442\u043e\u0440\u043d\u0430\u044f",
      "launch_behavior": "\u041f\u043e\u0432\u0435\u0434\u0435\u043d\u0438\u0435 \u0438 \u0417\u0430\u043f\u0443\u0441\u043a",
      "action_after_launch": "\u0414\u0435\u0439\u0441\u0442\u0432\u0438\u0435 \u043f\u043e\u0441\u043b\u0435 \u0437\u0430\u043f\u0443\u0441\u043a\u0430 \u0438\u0433\u0440\u044b",
      "action_behavior_desc": "\u041f\u043e\u0432\u0435\u0434\u0435\u043d\u0438\u0435 \u0433\u043b\u0430\u0432\u043d\u043e\u0433\u043e \u043e\u043a\u043d\u0430 \u043b\u0430\u0443\u043d\u0447\u0435\u0440\u0430 \u043f\u0440\u0438 \u0441\u0442\u0430\u0440\u0442\u0435 Minecraft",
      "action_minimize": "\u0421\u0432\u043e\u0440\u0430\u0447\u0438\u0432\u0430\u0442\u044c \u0432 \u0442\u0440\u0435\u0439",
      "action_close": "\u0417\u0430\u043a\u0440\u044b\u0432\u0430\u0442\u044c \u043b\u0430\u0443\u043d\u0447\u0435\u0440",
      "action_keep": "\u041e\u0441\u0442\u0430\u0432\u043b\u044f\u0442\u044c \u043e\u0442\u043a\u0440\u044b\u0442\u044b\u043c",
      "directories_paths": "\u0414\u0438\u0440\u0435\u043a\u0442\u043e\u0440\u0438\u0438 \u0438 \u041f\u0443\u0442\u0438",
      "custom_instances_folder": "\u041a\u0430\u0441\u0442\u043e\u043c\u043d\u0430\u044f \u043f\u0430\u043f\u043a\u0430 \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u043e\u0432",
      "btn_browse": "\u041e\u0431\u0437\u043e\u0440...",
      "btn_reset": "\u0421\u0431\u0440\u043e\u0441\u0438\u0442\u044c",
      "default_jvm_args": "Java \u0430\u0440\u0433\u0443\u043c\u0435\u043d\u0442\u044b \u043f\u043e \u0443\u043c\u043e\u043b\u0447\u0430\u043d\u0438\u044e",
      "jvm_args_desc": "\u0423\u043d\u0438\u0432\u0435\u0440\u0441\u0430\u043b\u044c\u043d\u044b\u0435 \u0444\u043b\u0430\u0433\u0438 \u0434\u043b\u044f \u0432\u0441\u0435\u0445 \u0432\u0435\u0440\u0441\u0438\u0439",
      "jvm_args_info": "\u041e\u043f\u0442\u0438\u043c\u0438\u0437\u0438\u0440\u043e\u0432\u0430\u043d\u043d\u044b\u0435 \u0430\u0440\u0433\u0443\u043c\u0435\u043d\u0442\u044b JVM. \u041f\u043e\u043b\u044c\u0437\u043e\u0432\u0430\u0442\u0435\u043b\u044c \u043c\u043e\u0436\u0435\u0442 \u0441\u0432\u043e\u0431\u043e\u0434\u043d\u043e \u0438\u0477\u043c\u0435\u043d\u044f\u0442\u044c \u0438\u0445 \u043f\u0440\u0438 \u043d\u0435\u043e\u0431\u0445\u043e\u0434\u0438\u043c\u043e\u0441\u0442\u0438:",
      "ui_animations": "\u0410\u043d\u0438\u043c\u0430\u0446\u0438\u0438 \u0438\u043d\u0442\u0435\u0440\u0444\u0435\u0439\u0441\u0430",
      "ui_anim_desc": "\u041f\u043b\u0430\u0432\u043d\u044b\u0435 \u043f\u0435\u0440\u0435\u0445\u043e\u0434\u044b \u0438 \u044d\u0444\u0444\u0435\u043a\u0442\u044b \u043d\u0430\u0432\u0435\u0434\u0435\u043d\u0438\u044f",
      "auto_updates": "\u0410\u0432\u0442\u043e\u043f\u0440\u043e\u0432\u0435\u0440\u043a\u0430 \u043e\u0431\u043d\u043e\u0432\u043b\u0435\u043d\u0438\u0439",
      "auto_updates_desc": "\u0420\u0435\u0433\u0443\u043b\u044f\u0440\u043d\u043e \u043f\u0440\u043e\u0432\u0435\u0440\u044f\u0442\u044c \u043d\u043e\u0432\u044b\u0435 \u0432\u0435\u0440\u0441\u0438\u0438 \u043a\u043e\u043c\u043f\u043e\u043d\u0435\u043d\u0442\u043e\u0432",
      "btn_save_settings": "\u0421\u043e\u0445\u0440\u0430\u043d\u0438\u0442\u044c \u043d\u0430\u0441\u0442\u0440\u043e\u0439\u043a\u0438",

      // Wizard
      "wizard_title": "\u0421\u043e\u0437\u0434\u0430\u043d\u0438\u0435 \u043d\u043e\u0432\u043e\u0433\u043e \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u0430",
      "wizard_name": "\u0418\u043c\u044f:",
      "wizard_group": "\u0413\u0440\u0443\u043f\u043f\u0430:",
      "wizard_custom": "\u041f\u043e\u043b\u044c\u0437\u043e\u0432\u0430\u0442\u0435\u043b\u044c\u0441\u043a\u0438\u0439",
      "wizard_import": "\u0418\u043c\u043f\u043e\u0440\u0442",
      "wizard_modrinth": "Modrinth",
      "wizard_version": "\u0412\u0435\u0440\u0441\u0438\u044f",
      "wizard_released": "\u0412\u044b\u043f\u0443\u0449\u0435\u043d\u043e",
      "wizard_type": "\u0422\u0438\u043f",
      "wizard_show": "\u041f\u043e\u043a\u0430\u0437\u044b\u0432\u0430\u0442\u044c",
      "wizard_releases": "\u0420\u0435\u043b\u0438\u0437\u044b",
      "wizard_snapshots": "\u0421\u043d\u0430\u043f\u0448\u043e\u0442\u044b",
      "wizard_betas": "\u0411\u0435\u0442\u044b",
      "wizard_alphas": "\u0410\u043b\u044c\u0444\u044b",
      "wizard_refresh": "\u041e\u0431\u043d\u043e\u0432\u0438\u0442\u044c",
      "wizard_modloader": "\u0417\u0430\u0433\u0440\u0443\u0437\u0447\u0438\u043a \u043c\u043e\u0434\u043e\u0432",
      "wizard_none": "\u041d\u0435\u0442",
      "wizard_search": "\u041f\u043e\u0438\u0441\u043a...",
      "wizard_btn_create": "\u0421\u043e\u0437\u0434\u0430\u0442\u044c",
      "wizard_btn_cancel": "\u041e\u0442\u043c\u0435\u043d\u0430",
      "wizard_drag_archive": "\u041f\u0435\u0440\u0435\u0442\u0430\u0449\u0438\u0442\u0435 \u0430\u0440\u0445\u0438\u0432 \u0441\u044e\u0434\u0430",
      "wizard_or_browse": "\u0438\u043b\u0438 \u043d\u0430\u0436\u043c\u0438\u0442\u0435 \u043a\u043d\u043e\u043f\u043a\u0443 \u043d\u0438\u0436\u0435 \u0434\u043b\u044f \u0432\u044b\u0431\u043e\u0440\u0430 \u0444\u0430\u0439\u043b\u0430",
      "wizard_pick_file": "\u0412\u044b\u0431\u0440\u0430\u0442\u044c \u0444\u0430\u0439\u043b",
      "wizard_search_modrinth": "\u041f\u043e\u0438\u0441\u043a \u043c\u043e\u0434\u043f\u0430\u043a\u043e\u0432 \u043d\u0430 Modrinth...",
      "all_loaders_short": "\u0412\u0441\u0435 \u043b\u043e\u0430\u0434\u0435\u0440\u044b",
      "wizard_select_modpack": "\u0412\u044b\u0431\u0435\u0440\u0438\u0442\u0435 \u043c\u043e\u0434\u043f\u0430\u043a \u0438\u0437 \u0441\u043f\u0438\u0441\u043a\u0430",
      "wizard_enter_query": "\u0412\u0422\u0435\u0434\u0438\u0442\u0435 \u0437\u0430\u043f\u0440\u043e\u0441 \u0434\u043b\u044f \u043f\u043e\u0438\u0441\u043a\u0430",
      "wizard_loading": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430...",
      "wizard_no_loader": "\u041d\u0435 \u0432\u044b\u0431\u0440\u0430\u043d \u0437\u0430\u0433\u0440\u0443\u0437\u0447\u0438\u043a \u043c\u043e\u0434\u043e\u0432.",
      "wizard_name_placeholder": "\u041d\u0430\u0437\u0432\u0430\u043d\u0438\u0435 \u0441\u0431\u043e\u0440\u043a\u0438...",
      "wizard_group_placeholder": "\u0411\u0435\u0437 \u0433\u0440\u0443\u043f\u043f\u044b",

      // Instance settings modal
      "settings_modal_title": "\u041d\u0430\u0441\u0442\u0440\u043e\u0439\u043a\u0438 \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u0430",
      "tab_general": "\u041e\u0441\u043d\u043e\u0432\u043d\u044b\u0435",
      "tab_java": "Java",
      "tab_minecraft": "Minecraft",
      "tab_mods": "\u041c\u043e\u0434\u044b",
      "tab_resourcepacks": "\u041d\u0430\u0431\u043e\u0440\u044b \u0440\u0435\u0441\u0443\u0440\u0441\u043e\u0432",
      "tab_shaders": "\u041d\u0430\u0431\u043e\u0440\u044b \u0448\u0435\u0439\u0434\u0435\u0440\u043e\u0432",
      "tab_worlds": "\u041c\u0438\u0440\u044b",
      "tab_servers": "\u0421\u0435\u0440\u0432\u0435\u0440\u044b",
      "settings_general_title": "\u041e\u0441\u043d\u043e\u0432\u043d\u044b\u0435 \u043d\u0430\u0441\u0442\u0440\u043e\u0439\u043a\u0438",
      "settings_icon": "\u0418\u043a\u043e\u043d\u043a\u0430",
      "settings_icon_desc": "\u041a\u0430\u0441\u0442\u043e\u043c\u043d\u043e\u0435 \u0438\u0437\u043e\u0431\u0440\u0430\u0436\u0435\u043d\u0438\u0435 \u0434\u043b\u044f \u0441\u0431\u043e\u0440\u043a\u0438",
      "settings_change_icon": "\u0421\u043c\u0435\u043d\u0438\u0442\u044c \u0438\u043a\u043e\u043d\u043a\u0443",
      "settings_instance_name": "\u041d\u0430\u0437\u0432\u0430\u043d\u0438\u0435 \u0441\u0431\u043e\u0440\u043a\u0438",
      "settings_instance_name_desc": "\u041e\u0442\u043e\u0431\u0440\u0430\u0436\u0430\u0435\u043c\u043e\u0435 \u0438\u043c\u044f \u0442\u0435\u043a\u0443\u0449\u0435\u0433\u043e \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u0430",
      "settings_group": "\u0413\u0440\u0443\u043f\u043f\u0430",
      "settings_group_desc": "\u041a\u0430\u0442\u0435\u0433\u043e\u0440\u0438\u044f \u0434\u043b\u044f \u0433\u0440\u0443\u043f\u043f\u0438\u0440\u043e\u0432\u043a\u0438 \u0441\u0431\u043e\u0440\u043e\u043a",
      "settings_mc_version": "\u0412\u0435\u0440\u0441\u0438\u044f Minecraft",
      "settings_mc_version_desc": "\u0412\u0435\u0440\u0441\u0438\u044f \u0438\u0433\u0440\u044b \u0442\u0435\u043a\u0443\u0449\u0435\u0433\u043e \u044d\u043a\u0437\u0435\u043c\u043f\u043b\u044f\u0440\u0430",
      "settings_loader_label": "\u0417\u0430\u0433\u0440\u0443\u0437\u0447\u0438\u043a \u043c\u043e\u0434\u043e\u0432",
      "settings_java_title": "\u041d\u0430\u0441\u0442\u0440\u043e\u0439\u043a\u0438 Java \u0438 \u041f\u0430\u043c\u044f\u0442\u0438",
      "settings_min_ram": "\u041c\u0438\u043d. \u041f\u0430\u043c\u044f\u0442\u044c (\u041c\u0411)",
      "settings_max_ram": "\u041c\u0430\u043a\u0441. \u041f\u0430\u043c\u044f\u0442\u044c (\u041c\u0411)",
      "settings_jvm_extra": "\u0414\u043e\u043f\u043e\u043b\u043d\u0438\u0442\u0435\u043b\u044c\u043d\u044b\u0435 JVM \u0430\u0440\u0433\u0443\u043c\u0435\u043d\u0442\u044b",
      "settings_jvm_extra_desc": "\u0414\u043e\u043f\u043e\u043b\u043d\u0438\u0442\u0435\u043b\u044c\u043d\u044b\u0435 \u0444\u043b\u0430\u0433\u0438 (\u0431\u0435\u0437 -Xms/-Xmx)",
      "settings_mc_title": "\u041f\u0430\u0440\u0430\u043c\u0435\u0442\u0440\u044b \u0437\u0430\u043f\u0443\u0441\u043a\u0430 Minecraft",
      "settings_width": "\u0428\u0438\u0440\u0438\u043d\u0430 \u043e\u043a\u043d\u0430 (px)",
      "settings_height": "\u0412\u044b\u0441\u043e\u0442\u0430 \u043e\u043a\u043d\u0430 (px)",
      "settings_fullscreen": "\u041f\u043e\u043b\u043d\u043e\u044d\u043a\u0440\u0430\u043d\u043d\u044b\u0439 \u0440\u0435\u0436\u0438\u043c",
      "settings_fullscreen_desc": "\u0417\u0430\u043f\u0443\u0441\u043a\u0430\u0442\u044c \u0438\u0433\u0440\u0443 \u0432\u043e \u0432\u0435\u0441\u044c \u044d\u043a\u0440\u0430\u043d",

      // Mods tab
      "tab_installed_mods": "\u0423\u0441\u0442\u0430\u043d\u043e\u0432\u043b\u0435\u043d\u043d\u044b\u0435 \u043c\u043e\u0434\u044b",
      "mods_installed_count": "\u0423\u0441\u0442\u0430\u043d\u043e\u0432\u043b\u0435\u043d\u043e:",
      "mods_active_count": "\u0410\u043a\u0442\u0438\u0432\u043d\u043e:",
      "btn_add_mod_file": "\u0414\u043e\u0431\u0430\u0432\u0438\u0442\u044c \u0444\u0430\u0439\u043b",
      "btn_folder": "\u041f\u0430\u043f\u043a\u0430",
      "btn_download": "\u0421\u043a\u0430\u0447\u0430\u0442\u044c",
      "mods_col_mod": "\u041c\u043e\u0434",
      "mods_col_status": "\u0421\u0442\u0430\u0442\u0443\u0441",
      "mods_col_actions": "\u0414\u0435\u0439\u0441\u0442\u0432\u0438\u044f",
      "loading_mods": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430 \u043c\u043e\u0434\u043e\u0432...",
      "search_modrinth": "\u041f\u043e\u0438\u0441\u043a \u043d\u0430 Modrinth...",
      "search_enter_query": "\u0412\u0432\u0435\u0434\u0438\u0442\u0435 \u0437\u0430\u043f\u0440\u043e\u0441 \u0434\u043b\u044f \u043f\u043e\u0438\u0441\u043a\u0430",

      // Mod drawer
      "drawer_mod_version": "\u0412\u0435\u0440\u0441\u0438\u044f \u043c\u043e\u0434\u0430",
      "drawer_loading": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430...",
      "btn_download_mod": "\u0421\u043a\u0430\u0447\u0430\u0442\u044c \u043c\u043e\u0434",

      // Resource packs tab
      "tab_rp_title": "\u041d\u0430\u0431\u043e\u0440\u044b \u0440\u0435\u0441\u0443\u0440\u0441\u043e\u0432",
      "tab_rp_subtitle": "\u0423\u0441\u0442\u0430\u043d\u043e\u0432\u043b\u0435\u043d\u043d\u044b\u0435 \u0442\u0435\u043a\u0441\u0442\u0443\u0440-\u043f\u0430\u043a\u0438",
      "loading_rp": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430 \u0440\u0435\u0441\u0443\u0440\u0441-\u043f\u0430\u043a\u043e\u0432...",
      "no_rp": "\u041d\u0435\u0442 \u0443\u0441\u0442\u0430\u043d\u043e\u0432\u043b\u0435\u043d\u043d\u044b\u0445 \u0440\u0435\u0441\u0443\u0440\u0441-\u043f\u0430\u043a\u043e\u0432",

      // Shaders tab
      "tab_shaders_title": "\u041d\u0430\u0431\u043e\u0440\u044b \u0448\u0435\u0439\u0434\u0435\u0440\u043e\u0432",
      "tab_shaders_subtitle": "\u0423\u0441\u0442\u0430\u043d\u043e\u0432\u043b\u0435\u043d\u043d\u044b\u0435 \u0433\u0440\u0430\u0444\u0438\u0447\u0435\u0441\u043a\u0438\u0435 \u0448\u0435\u0439\u0434\u0435\u0440\u044b",
      "loading_shaders": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430 \u0448\u0435\u0439\u0434\u0435\u0440\u043e\u0432...",
      "no_shaders": "\u041d\u0435\u0442 \u0443\u0441\u0442\u0430\u043d\u043e\u0432\u043b\u0435\u043d\u043d\u044b\u0445 \u0448\u0435\u0439\u0434\u0435\u0440\u043e\u0432",

      // Worlds tab
      "tab_worlds_title": "\u041c\u0438\u0440\u044b",
      "tab_worlds_subtitle": "\u0421\u043e\u0445\u0440\u0430\u043d\u0435\u043d\u043d\u044b\u0435 \u043b\u043e\u043a\u0430\u043b\u044c\u043d\u044b\u0435 \u043c\u0438\u0440\u044b",
      "btn_import_world": "\u0418\u043c\u043f\u043e\u0440\u0442\u0438\u0440\u043e\u0432\u0430\u0442\u044c \u043c\u0438\u0440 (.zip)",
      "loading_worlds": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430 \u043c\u0438\u0440\u043e\u0432...",
      "no_worlds": "\u041d\u0435\u0442 \u0441\u043e\u0445\u0440\u0430\u043d\u0435\u043d\u043d\u044b\u0445 \u043c\u0438\u0440\u043e\u0432",

      // Servers tab
      "tab_servers_title": "\u0421\u0435\u0440\u0432\u0435\u0440\u044b",
      "tab_servers_subtitle": "\u0421\u043e\u0445\u0440\u0430\u043d\u0435\u043d\u043d\u044b\u0435 \u0441\u0435\u0442\u0435\u0432\u044b\u0435 \u0441\u0435\u0440\u0432\u0435\u0440\u0430 (servers.dat)",
      "loading_servers": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430 \u0441\u0435\u0440\u0432\u0435\u0440\u043e\u0432...",
      "no_servers": "\u041d\u0435\u0442 \u0441\u043e\u0445\u0440\u0430\u043d\u0435\u043d\u043d\u044b\u0445 \u0441\u0435\u0440\u0432\u0435\u0440\u043e\u0432",

      // Modrinth tab
      "tab_modrinth_title": "\u0418\u043d\u0442\u0435\u0433\u0440\u0430\u0446\u0438\u044f Modrinth",
      "modrinth_pack_updates": "\u041e\u0431\u043d\u043e\u0432\u043b\u0435\u043d\u0438\u044f \u0441\u0431\u043e\u0440\u043a\u0438",
      "modrinth_updates_desc": "\u041f\u0440\u043e\u0432\u0435\u0440\u043a\u0430 \u043d\u043e\u0432\u043e\u0439 \u0432\u0435\u0440\u0441\u0438\u0438 \u0441\u0431\u043e\u0440\u043a\u0438 \u043d\u0430 \u043f\u043b\u0430\u0442\u0444\u043e\u0440\u043c\u0435 Modrinth",
      "btn_check_updates": "\u041f\u0440\u043e\u0432\u0435\u0440\u0438\u0442\u044c \u043e\u0431\u043d\u043e\u0432\u043b\u0435\u043d\u0438\u044f",

      // Screenshots tab
      "tab_screenshots_title": "\u0421\u043d\u0438\u043c\u043a\u0438 \u044d\u043a\u0440\u0430\u043d\u0430",
      "tab_screenshots_desc": "\u0421\u043a\u0440\u0438\u043d\u0448\u043e\u0442\u044b.",

      // Mods browser page
      "mods_catalog": "\u041a\u0430\u0442\u0430\u043b\u043e\u0433 \u043c\u043e\u0434\u043e\u0432",
      "mods_subtitle": "\u041f\u043e\u0438\u0441\u043a \u0438 \u0437\u0430\u0433\u0440\u0443\u0437\u043a\u0430 \u043c\u043e\u0434\u0438\u0444\u0438\u043a\u0430\u0446\u0438\u0439 \u0441 Modrinth",
      "target_build": "\u0426\u0435\u043b\u0435\u0432\u0430\u044f \u0441\u0431\u043e\u0440\u043a\u0430: \u0422\u0435\u043a\u0443\u0449\u0430\u044f",
      "all_loaders": "\u0412\u0441\u0435 \u0437\u0430\u0433\u0440\u0443\u0437\u0447\u0438\u043a\u0438",
      "search_mods": "\u041f\u043e\u0438\u0441\u043a \u043c\u043e\u0434\u043e\u0432...",
      "loading_mods_catalog": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430 \u043a\u0430\u0442\u0430\u043b\u043e\u0433\u0430 \u043c\u043e\u0434\u043e\u0432...",

      // Add Account modal
      "add_account_title": "\u0414\u043e\u0431\u0430\u0432\u0438\u0442\u044c \u0430\u043a\u043a\u0430\u0443\u043d\u0442",
      "msa_login_desc": "\u0412\u043e\u0439\u0442\u0438 \u0447\u0435\u0440\u0435\u0437 Microsoft (MSA)",
      "offline_label": "\u041e\u0444\u043b\u0430\u0439\u043d",
      "offline_desc": "\u041f\u0440\u043e\u0438\u0437\u0432\u043e\u043b\u044c\u043d\u044b\u0439 \u043d\u0438\u043a, \u0431\u0435\u0437 \u043b\u0438\u0446\u0435\u043d\u0437\u0438\u0438",
      "offline_modal_title": "\u041e\u0444\u043b\u0430\u0439\u043d-\u0430\u043a\u043a\u0430\u0443\u043d\u0442",
      "offline_modal_subtitle": "\u0412\u0432\u0435\u0434\u0438\u0442\u0435 \u0436\u0435\u043b\u0430\u0435\u043c\u044b\u0439 \u043d\u0438\u043a\u043d\u0435\u0439\u043c",
      "btn_add": "\u0414\u043e\u0431\u0430\u0432\u0438\u0442\u044c",
      "btn_cancel": "\u041e\u0442\u043c\u0435\u043d\u0430",
      "btn_save": "\u0421\u043e\u0445\u0440\u0430\u043d\u0438\u0442\u044c",
      "btn_close": "\u0417\u0430\u043a\u0440\u044b\u0442\u044c",

      // Alert modal
      "alert_notification": "\u0423\u0432\u0435\u0434\u043e\u043c\u043b\u0435\u043d\u0438\u0435",

      // MSA modal
      "msa_title": "\u0410\u0432\u0442\u043e\u0440\u0438\u0437\u0430\u0446\u0438\u044f Microsoft",
      "msa_subtitle": "\u0414\u043b\u044f \u0432\u0445\u043e\u0434\u0430 \u043f\u0435\u0440\u0435\u0439\u0434\u0438\u0442\u0435 \u043f\u043e \u0441\u0441\u044b\u043b\u043a\u0435 \u043d\u0438\u0436\u0435 \u0438 \u0432\u0432\u0435\u0434\u0438\u0442\u0435 \u0443\u043a\u0430\u0437\u0430\u043d\u043d\u044b\u0439 \u043a\u043e\u0434:",
      "msa_confirm_code": "\u041a\u043e\u0434 \u043f\u043e\u0434\u0442\u0432\u0435\u0440\u0436\u0434\u0435\u043d\u0438\u044f",
      "msa_waiting": "\u041e\u0436\u0438\u0434\u0430\u043d\u0438\u0435 \u0432\u0445\u043e\u0434\u0430 \u0432 \u0430\u043a\u043a\u0430\u0443\u043d\u0442...",

      // Download progress box
      "dl_current_process": "\u0422\u0435\u043a\u0443\u0449\u0438\u0439 \u043f\u0440\u043e\u0446\u0435\u0441\u0441",
      "dl_loading": "\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430 \u0440\u0435\u0441\u0443\u0440\u0441\u043e\u0432...",
      "dl_speed": "\u0421\u043a\u043e\u0440\u043e\u0441\u0442\u044c",
      "dl_remaining": "\u041e\u0441\u0442\u0430\u043b\u043e\u0441\u044c",
      "dl_threads": "\u041f\u043e\u0442\u043e\u043a\u0438",

      // Footer
      "status_ready": "\u0413\u043e\u0442\u043e\u0432 \u043a \u0440\u0430\u0431\u043e\u0442\u0435"
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

      // Languages
      "lang_ru": "Русский (Russian)",
      "lang_en": "English (Английский)",

      // Themes
      "theme_dark_title": "Prism Dark (Default)",
      "theme_dark_desc": "Dark theme with purple accent",
      "theme_onyx_title": "Midnight Onyx",
      "theme_onyx_desc": "Deep black color",
      "theme_light_title": "Cloud White",
      "theme_light_desc": "Light clean theme",
      "theme_ember_title": "Ember Flow",
      "theme_ember_desc": "Warm orange tones",

      // Instances Page
      "my_instances": "My Instances",
      "manage_instances": "Manage your Minecraft instances",
      "total_instances": "Total instances:",
      "no_instances_found": "No instances found",
      "create_or_import": "Create a new instance or import an existing build.",
      "create_instance": "Create Instance",
      "import_zip": "Import ZIP",
      "unassigned_group": "Unassigned",
      "no_instances_title": "No instances found",
      "no_instances_desc": "Create a new instance or import an existing build.",
      "btn_create_first": "Create Instance",
      "btn_import_first": "Import ZIP",

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
      "add_account_placeholder": "Add Account",

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

      // Wizard
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
      "wizard_drag_archive": "Drop archive here",
      "wizard_or_browse": "or click the button below to choose a file",
      "wizard_pick_file": "Choose File",
      "wizard_search_modrinth": "Search modpacks on Modrinth...",
      "all_loaders_short": "All Loaders",
      "wizard_select_modpack": "Select a modpack from the list",
      "wizard_enter_query": "Type to search",
      "wizard_loading": "Loading...",
      "wizard_no_loader": "No mod loader selected.",
      "wizard_name_placeholder": "Instance name...",
      "wizard_group_placeholder": "No group",

      // Instance settings modal
      "settings_modal_title": "Instance Settings",
      "tab_general": "General",
      "tab_java": "Java",
      "tab_minecraft": "Minecraft",
      "tab_mods": "Mods",
      "tab_resourcepacks": "Resource Packs",
      "tab_shaders": "Shader Packs",
      "tab_worlds": "Worlds",
      "tab_servers": "Servers",
      "settings_general_title": "General Settings",
      "settings_icon": "Icon",
      "settings_icon_desc": "Custom image for this instance",
      "settings_change_icon": "Change Icon",
      "settings_instance_name": "Instance Name",
      "settings_instance_name_desc": "Display name of the current instance",
      "settings_group": "Group",
      "settings_group_desc": "Category for grouping instances",
      "settings_mc_version": "Minecraft Version",
      "settings_mc_version_desc": "Game version for this instance",
      "settings_loader_label": "Mod Loader",
      "settings_java_title": "Java & Memory Settings",
      "settings_min_ram": "Min Memory (MB)",
      "settings_max_ram": "Max Memory (MB)",
      "settings_jvm_extra": "Extra JVM Arguments",
      "settings_jvm_extra_desc": "Additional flags (excluding -Xms/-Xmx)",
      "settings_mc_title": "Minecraft Launch Options",
      "settings_width": "Window Width (px)",
      "settings_height": "Window Height (px)",
      "settings_fullscreen": "Fullscreen Mode",
      "settings_fullscreen_desc": "Launch game in fullscreen",

      // Mods tab
      "tab_installed_mods": "Installed Mods",
      "mods_installed_count": "Installed:",
      "mods_active_count": "Active:",
      "btn_add_mod_file": "Add File",
      "btn_folder": "Folder",
      "btn_download": "Download",
      "mods_col_mod": "Mod",
      "mods_col_status": "Status",
      "mods_col_actions": "Actions",
      "loading_mods": "Loading mods...",
      "search_modrinth": "Search on Modrinth...",
      "search_enter_query": "Type to search",

      // Mod drawer
      "drawer_mod_version": "Mod Version",
      "drawer_loading": "Loading...",
      "btn_download_mod": "Download Mod",

      // Resource packs tab
      "tab_rp_title": "Resource Packs",
      "tab_rp_subtitle": "Installed texture packs",
      "loading_rp": "Loading resource packs...",
      "no_rp": "No installed resource packs",

      // Shaders tab
      "tab_shaders_title": "Shader Packs",
      "tab_shaders_subtitle": "Installed graphical shaders",
      "loading_shaders": "Loading shaders...",
      "no_shaders": "No installed shader packs",

      // Worlds tab
      "tab_worlds_title": "Worlds",
      "tab_worlds_subtitle": "Saved local worlds",
      "btn_import_world": "Import World (.zip)",
      "loading_worlds": "Loading worlds...",
      "no_worlds": "No saved worlds",

      // Servers tab
      "tab_servers_title": "Servers",
      "tab_servers_subtitle": "Saved multiplayer servers (servers.dat)",
      "loading_servers": "Loading servers...",
      "no_servers": "No saved servers",

      // Modrinth tab
      "tab_modrinth_title": "Modrinth Integration",
      "modrinth_pack_updates": "Pack Updates",
      "modrinth_updates_desc": "Check for a new version of this pack on Modrinth",
      "btn_check_updates": "Check for Updates",

      // Screenshots tab
      "tab_screenshots_title": "Screenshots",
      "tab_screenshots_desc": "Game screenshots.",

      // Mods browser page
      "mods_catalog": "Mods Catalog",
      "mods_subtitle": "Search and download mods from Modrinth",
      "target_build": "Target Build: Current",
      "all_loaders": "All Loaders",
      "search_mods": "Search mods...",
      "loading_mods_catalog": "Loading mods catalog...",

      // Add Account modal
      "add_account_title": "Add Account",
      "msa_login_desc": "Sign in via Microsoft (MSA)",
      "offline_label": "Offline",
      "offline_desc": "Custom username, no license required",
      "offline_modal_title": "Offline Account",
      "offline_modal_subtitle": "Enter your desired username",
      "btn_add": "Add",
      "btn_cancel": "Cancel",
      "btn_save": "Save",
      "btn_close": "Close",

      // Alert modal
      "alert_notification": "Notification",

      // MSA modal
      "msa_title": "Microsoft Sign In",
      "msa_subtitle": "Visit the link below and enter the code shown:",
      "msa_confirm_code": "Confirmation Code",
      "msa_waiting": "Waiting for browser login...",

      // Download progress box
      "dl_current_process": "Current Process",
      "dl_loading": "Downloading assets...",
      "dl_speed": "Speed",
      "dl_remaining": "Remaining",
      "dl_threads": "Threads",

      // Footer
      "status_ready": "Ready"
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
