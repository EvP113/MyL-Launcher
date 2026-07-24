// backend-api.js — Tauri API wrapper (regular script, no ES modules)

(function () {

  function getInvoke() {
    return window.__TAURI__ && window.__TAURI__.core
      ? window.__TAURI__.core.invoke
      : null;
  }

  function getListen() {
    return window.__TAURI__ && window.__TAURI__.event
      ? window.__TAURI__.event.listen
      : null;
  }

  function getWin() {
    if (window.__TAURI__ && window.__TAURI__.window) {
      return window.__TAURI__.window.getCurrentWindow();
    }
    return null;
  }

  // Window controls - call at runtime, not load time
  window.windowControls = {
    minimize: function () { var w = getWin(); if (w) w.minimize(); },
    toggleMaximize: function () { var w = getWin(); if (w) w.toggleMaximize(); },
    close: function () { var w = getWin(); if (w) w.close(); },
  };

  function callInvoke(name, args) {
    var fn = getInvoke();
    if (!fn) return Promise.reject('Tauri not available');
    return fn(name, args || {});
  }

  window.backend = {
    // Instances
    listInstances: function () { return callInvoke('list_instances'); },
    createInstance: function (params) { return callInvoke('create_instance', { paramsJson: JSON.stringify(params) }); },
    updateInstanceConfig: function (params) { return callInvoke('update_instance_config', { paramsJson: JSON.stringify(params) }); },
    deleteInstance: function (instanceId) { return callInvoke('delete_instance', { instanceId: instanceId }); },
    getInstanceConfig: function (instanceId) { return callInvoke('get_instance_config', { instanceId: instanceId }); },
    setInstanceIcon: function (instanceId) { return callInvoke('set_instance_icon', { instanceId: instanceId }); },
    listResourcepacks: function (instanceId) { return callInvoke('list_resourcepacks', { instanceId: instanceId }); },
    listShaderpacks: function (instanceId) { return callInvoke('list_shaderpacks', { instanceId: instanceId }); },
    listWorlds: function (instanceId) { return callInvoke('list_worlds', { instanceId: instanceId }); },
    importWorldZip: function (instanceId) { return callInvoke('import_world_zip', { instanceId: instanceId }); },
    listServers: function (instanceId) { return callInvoke('list_servers', { instanceId: instanceId }); },
    deleteResourcepack: function (instanceId, filename) { return callInvoke('delete_resourcepack', { instanceId: instanceId, filename: filename }); },
    deleteShaderpack: function (instanceId, filename) { return callInvoke('delete_shaderpack', { instanceId: instanceId, filename: filename }); },
    deleteWorld: function (instanceId, folderName) { return callInvoke('delete_world', { instanceId: instanceId, folderName: folderName }); },
    launchInstance: function (instanceId, accountId) { return callInvoke('launch_instance', { instanceId: instanceId, accountId: accountId }); },
    stopGame: function () { return callInvoke('stop_game'); },
    stopGameByPid: function (pid) { return callInvoke('stop_game_by_pid', { pid: pid }); },
    openFolder: function (path) { return callInvoke('open_folder', { path: path }); },
    ensureJava: function (mcVersion) { return callInvoke('ensure_java', { paramsJson: JSON.stringify({ mc_version: mcVersion }) }); },
    // Accounts
    listAccounts: function () { return callInvoke('list_accounts'); },
    addAccountOffline: function (username) { return callInvoke('add_account_offline', { username: username }); },
    removeAccount: function (accountId) { return callInvoke('remove_account', { accountId: accountId }); },
    setActiveAccount: function (accountId) { return callInvoke('set_active_account', { accountId: accountId }); },
    getAccount: function (accountId) { return callInvoke('get_account', { accountId: accountId }); },
    loginMsaPkce: function () { return callInvoke('login_msa_pkce'); },
    startMsaAuth: function () { return callInvoke('start_msa_auth'); },
    pollMsaAuth: function (deviceCode) { return callInvoke('poll_msa_auth', { deviceCode: deviceCode }); },
    refreshMsaAccount: function (accountId) { return callInvoke('refresh_msa_account', { accountId: accountId }); },
    // Mods
    searchMods: function (query, mcVersion, loader, page) { return callInvoke('search_mods', { query: query, mcVersion: mcVersion, loader: loader, page: page }); },
    getModVersions: function (slug, mcVersion, loader) { return callInvoke('get_mod_versions', { slug: slug, mcVersion: mcVersion, loader: loader }); },
    downloadMod: function (url, instanceId, filename) { return callInvoke('download_mod', { url: url, instanceId: instanceId, filename: filename }); },
    listInstalledMods: function (instanceId) { return callInvoke('list_installed_mods', { instanceId: instanceId }); },
    toggleMod: function (instanceId, filename, enable) { return callInvoke('toggle_mod', { instanceId: instanceId, filename: filename, enable: enable }); },
    deleteMod: function (instanceId, filename) { return callInvoke('delete_mod', { instanceId: instanceId, filename: filename }); },
    addModFile: function (instanceId, srcPath) { return callInvoke('add_mod_file', { instanceId: instanceId, srcPath: srcPath }); },
    // Versions
    getModLoaders: function (mcVersion) { return callInvoke('get_mod_loaders', { mcVersion: mcVersion }); },
    getMcVersions: function (typeFilter) { return callInvoke('get_mc_versions', { typeFilter: typeFilter }); },
    // Import/Export
    importInstance: function (filePath) { return callInvoke('import_instance', { filePath: filePath }); },
    openFileDialog: function () { return callInvoke('open_file_dialog'); },
    // Shortcuts
    createShortcut: function (instanceId, targetPath) { return callInvoke('create_shortcut', { instanceId: instanceId, targetPath: targetPath }); },
    // Modpacks (Modrinth)
    searchModpacks: function (query, loader, page) { return callInvoke('search_modpacks', { query: query, loader: loader || '', page: page || 0 }); },
    getModpackVersions: function (slug) { return callInvoke('get_modpack_versions', { slug: slug }); },
    importModpackFromUrl: function (url, filename, iconUrl) { return callInvoke('import_modpack_from_url', { url: url, filename: filename, iconUrl: iconUrl || null }); },
    // Event listeners
    onGameLog: function (callback) {
      return window.listenEvent('game-log', function (ev) {
        if (ev && ev.payload) {
          callback(ev.payload.line || ev.payload);
        }
      });
    },
    onDownloadProgress: function (callback) {
      return window.listenEvent('download-progress', function (ev) {
        if (ev && ev.payload) {
          callback(ev.payload);
        }
      });
    },
    onGameExit: function (callback) {
      return window.listenEvent('game-exit', function (ev) {
        if (ev && ev.payload) {
          callback(ev.payload);
        }
      });
    },
  };

  window.listenEvent = function (event, handler) {
    var fn = getListen();
    if (!fn) return Promise.resolve(function () {});
    return fn(event, handler);
  };
})();
