// app.js — SPA navigation, instances, accounts, mods, settings
(function () {
  // === Window controls ===
  document.getElementById('btn-minimize').addEventListener('click', function () { window.windowControls.minimize(); });
  document.getElementById('btn-maximize').addEventListener('click', function () { window.windowControls.toggleMaximize(); });
  document.getElementById('btn-close').addEventListener('click', function () { window.windowControls.close(); });

  // === App State ===
  var instanceDataMap = {};
  var selectedInstanceId = null;
  var runningInstancePid = null;
  var runningInstanceId = null;
  var currentViewMode = localStorage.getItem('myl_view_mode') || 'grid';

  // === SPA Navigation ===
  var navLinks = document.querySelectorAll('.nav-link');
  var pages = document.querySelectorAll('.page');
  function navigateTo(name) {
    pages.forEach(function (p) { p.classList.remove('active'); });
    navLinks.forEach(function (link) {
      var active = link.dataset.page === name;
      link.classList.toggle('bg-primary/10', active);
      link.classList.toggle('text-primary', active);
      link.classList.toggle('border-l-2', active);
      link.classList.toggle('border-primary', active);
      link.classList.toggle('font-bold', active);
      link.classList.toggle('rounded-r-lg', active);
      link.classList.toggle('text-text-secondary', !active);
      link.classList.toggle('hover:text-on-surface', !active);
      link.classList.toggle('hover:bg-surface-variant', !active);
      link.classList.toggle('rounded-lg', !active);
      var icon = link.querySelector('.material-symbols-outlined');
      if (icon) icon.style.fontVariationSettings = active ? "'FILL' 1" : "'FILL' 0";
    });
    var target = document.getElementById('page-' + name);
    if (target) { target.classList.add('active'); target.style.opacity = '0'; requestAnimationFrame(function () { target.style.opacity = '1'; }); }
    if (name === 'instances') loadInstances();
    if (name === 'accounts') loadAccounts();
    if (name === 'settings') initLauncherSettings();
  }
  navLinks.forEach(function (link) { link.addEventListener('click', function () { navigateTo(link.dataset.page); }); });

  // === Modal helpers ===
  function openModal(id) { var m = document.getElementById(id); if (m) { m.classList.remove('hidden'); m.classList.add('flex', 'modal-animate'); } }
  function closeModal(id) { var m = document.getElementById(id); if (m) { m.classList.add('hidden'); m.classList.remove('flex', 'modal-animate', 'modal-closing'); } }

  function showAlert(title, text, cancel, type) {
    var titleEl = document.getElementById('modal-alert-title');
    var textEl = document.getElementById('modal-alert-text');
    var cancelEl = document.getElementById('modal-alert-cancel');
    var okEl = document.getElementById('modal-alert-ok');
    var barEl = document.getElementById('modal-alert-bar');
    var iconBoxEl = document.getElementById('modal-alert-icon-box');
    var iconEl = document.getElementById('modal-alert-icon');
    var okIconEl = document.getElementById('modal-alert-ok-icon');
    var okTextEl = document.getElementById('modal-alert-ok-text');

    if (titleEl) titleEl.textContent = title;
    if (textEl) textEl.textContent = text;
    if (cancelEl) cancelEl.classList.toggle('hidden', !cancel);

    type = type || (cancel ? 'delete' : 'info');

    if (type === 'delete') {
      if (barEl) barEl.className = 'absolute top-0 left-0 w-full h-1 bg-status-error';
      if (iconBoxEl) iconBoxEl.className = 'flex-shrink-0 w-12 h-12 flex items-center justify-center rounded-full bg-status-error/10 text-status-error';
      if (iconEl) iconEl.textContent = 'warning';
      if (okEl) okEl.className = 'px-6 py-2.5 rounded-lg bg-status-error hover:bg-status-error/90 text-white font-label-md text-label-md shadow-lg shadow-status-error/20 transition-all active:scale-95 flex items-center gap-2';
      if (okIconEl) okIconEl.textContent = 'delete';
      if (okTextEl) okTextEl.textContent = 'Удалить';
    } else if (type === 'error') {
      if (barEl) barEl.className = 'absolute top-0 left-0 w-full h-1 bg-status-warning';
      if (iconBoxEl) iconBoxEl.className = 'flex-shrink-0 w-12 h-12 flex items-center justify-center rounded-full bg-status-warning/10 text-status-warning';
      if (iconEl) iconEl.textContent = 'error';
      if (okEl) okEl.className = 'px-6 py-2.5 rounded-lg bg-status-warning text-white font-label-md text-label-md shadow-lg shadow-status-warning/20 transition-all active:scale-95 flex items-center gap-2';
      if (okIconEl) okIconEl.textContent = 'check';
      if (okTextEl) okTextEl.textContent = 'OK';
    } else { // success / info
      if (barEl) barEl.className = 'absolute top-0 left-0 w-full h-1 bg-primary';
      if (iconBoxEl) iconBoxEl.className = 'flex-shrink-0 w-12 h-12 flex items-center justify-center rounded-full bg-primary/10 text-primary';
      if (iconEl) iconEl.textContent = type === 'success' ? 'check_circle' : 'info';
      if (okEl) okEl.className = 'px-6 py-2.5 rounded-lg bg-primary-container text-white font-label-md text-label-md shadow-lg shadow-primary/20 transition-all active:scale-95 flex items-center gap-2';
      if (okIconEl) okIconEl.textContent = 'check';
      if (okTextEl) okTextEl.textContent = 'OK';
    }

    return new Promise(function (resolve) {
      openModal('modal-alert');
      if (okEl) okEl.onclick = function () { closeModal('modal-alert'); resolve(true); };
      if (cancelEl) cancelEl.onclick = function () { closeModal('modal-alert'); resolve(false); };
    });
  }
  window.showAlert = showAlert;

  // === View Mode Buttons (Grid vs List) ===
  var btnViewGrid = document.getElementById('btn-view-grid');
  var btnViewList = document.getElementById('btn-view-list');

  function updateViewModeButtons() {
    if (btnViewGrid && btnViewList) {
      btnViewGrid.className = currentViewMode === 'grid' ? 'p-1.5 bg-surface-variant text-primary rounded shadow-sm' : 'p-1.5 text-text-secondary hover:text-on-surface';
      btnViewList.className = currentViewMode === 'list' ? 'p-1.5 bg-surface-variant text-primary rounded shadow-sm' : 'p-1.5 text-text-secondary hover:text-on-surface';
    }
  }

  function setViewMode(mode) {
    currentViewMode = mode;
    localStorage.setItem('myl_view_mode', mode);
    updateViewModeButtons();
    renderInstancesUI();
  }
  if (btnViewGrid) btnViewGrid.addEventListener('click', function () { setViewMode('grid'); });
  if (btnViewList) btnViewList.addEventListener('click', function () { setViewMode('list'); });

  // === Instances & Groups ===
  var rawInstancesList = [];

  function loadInstances() {
    window.backend.listInstances().then(function (json) {
      rawInstancesList = JSON.parse(json) || [];
      instanceDataMap = {};
      rawInstancesList.forEach(function (inst) { instanceDataMap[inst.id] = inst; });
      renderInstancesUI();
    }).catch(function (e) { console.warn(e); });
  }

  function updateTargetInstanceSelect() {
    var sel = document.getElementById('mod-target-instance');
    if (!sel) return;
    var currentVal = sel.value;
    sel.innerHTML = '';
    var defOpt = document.createElement('option');
    defOpt.value = '';
    defOpt.textContent = window.i18n ? window.i18n.t('target_build', 'Целевая сборка: Текущая') : 'Целевая сборка: Текущая';
    sel.appendChild(defOpt);
    Object.keys(instanceDataMap).forEach(function (id) {
      var inst = instanceDataMap[id];
      var opt = document.createElement('option');
      opt.value = id;
      opt.textContent = inst.name + ' (' + inst.mc_version + ')';
      if (id === currentVal) opt.selected = true;
      sel.appendChild(opt);
    });
  }

  function renderInstancesUI() {
    var container = document.getElementById('instances-container');
    var emptyState = document.getElementById('empty-state');
    var instanceGrid = document.getElementById('instance-grid');
    var countEl = document.getElementById('instance-count');

    if (!rawInstancesList || rawInstancesList.length === 0) {
      emptyState.classList.remove('hidden'); instanceGrid.classList.add('hidden');
      countEl.textContent = '0'; return;
    }
    emptyState.classList.add('hidden'); instanceGrid.classList.remove('hidden');
    countEl.textContent = String(rawInstancesList.length);
    updateViewModeButtons();
    container.innerHTML = '';

    // Determine initial selected instance
    var lastId = localStorage.getItem('myl_last_instance');
    if (!selectedInstanceId && lastId && instanceDataMap[lastId]) {
      selectedInstanceId = lastId;
    } else if (!selectedInstanceId && rawInstancesList.length > 0) {
      selectedInstanceId = rawInstancesList[0].id;
    }
    updateTargetInstanceSelect();

    // Group instances by group name
    var groups = {};
    var defaultGroupLabel = window.i18n ? window.i18n.t('unassigned_group', 'Без группы') : 'Без группы';
    rawInstancesList.forEach(function (inst) {
      var gName = inst.group && inst.group.trim() ? inst.group.trim() : defaultGroupLabel;
      if (!groups[gName]) groups[gName] = [];
      groups[gName].push(inst);
    });

    Object.keys(groups).sort().forEach(function (groupName) {
      var instList = groups[groupName];
      var groupSection = document.createElement('div');
      groupSection.className = 'space-y-3';

      var groupHeader = document.createElement('div');
      groupHeader.className = 'flex items-center gap-2 text-text-secondary font-label-md text-label-md border-b border-outline-variant/30 pb-2';
      groupHeader.innerHTML = '<span class="material-symbols-outlined text-[18px]">folder</span><span class="font-bold text-on-surface">' + esc(groupName) + '</span><span class="text-text-secondary font-normal text-[12px]">(' + instList.length + ')</span>';
      groupSection.appendChild(groupHeader);

      var itemsWrapper = document.createElement('div');
      if (currentViewMode === 'grid') {
        itemsWrapper.className = 'grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6';
        instList.forEach(function (inst) {
          var card = document.createElement('div');
          var isSelected = selectedInstanceId === inst.id;
          card.className = 'p-4 bg-surface-card rounded-xl border flex flex-col items-center gap-4 relative group cursor-pointer transition-all ' + (isSelected ? 'border-primary ring-2 ring-primary/20' : 'border-outline-variant hover:border-outline');
          card.dataset.id = inst.id;
          var iconHtmlGrid = inst.icon ? '<img src="' + esc(inst.icon) + '" class="w-24 h-24 object-cover rounded-2xl bg-surface-container-high shrink-0"/>' : '<div class="w-24 h-24 rounded-2xl bg-surface-container-high p-4 flex items-center justify-center shrink-0"><span class="material-symbols-outlined text-[40px] text-text-secondary">deployed_code</span></div>';
          card.innerHTML = iconHtmlGrid +
            '<div class="text-center"><h3 class="font-headline-md text-headline-md font-bold text-on-surface">' + esc(inst.name) + '</h3>' +
            '<div class="flex items-center justify-center gap-2 text-text-secondary font-label-md text-label-md mt-1"><span class="material-symbols-outlined text-[16px]">box</span>' + esc(inst.mc_version || '—') + (inst.loader ? ' (' + esc(inst.loader) + ')' : '') + '</div></div>';
          card.addEventListener('click', function () { selectInstance(inst.id); });
          itemsWrapper.appendChild(card);
        });
      } else {
        itemsWrapper.className = 'space-y-2';
        instList.forEach(function (inst) {
          var row = document.createElement('div');
          var isSelected = selectedInstanceId === inst.id;
          row.className = 'p-3 bg-surface-card rounded-xl border flex items-center justify-between gap-4 cursor-pointer transition-all ' + (isSelected ? 'border-primary ring-2 ring-primary/20' : 'border-outline-variant hover:border-outline');
          row.dataset.id = inst.id;
          var iconHtmlList = inst.icon ? '<img src="' + esc(inst.icon) + '" class="w-10 h-10 object-cover rounded-lg bg-surface-container-high shrink-0"/>' : '<div class="w-10 h-10 rounded-lg bg-surface-container-high flex items-center justify-center shrink-0"><span class="material-symbols-outlined text-text-secondary text-[24px]">deployed_code</span></div>';
          row.innerHTML = '<div class="flex items-center gap-3">' + iconHtmlList +
            '<div><h3 class="font-headline-md text-headline-md font-bold text-on-surface">' + esc(inst.name) + '</h3><div class="text-label-sm text-text-secondary flex items-center gap-2"><span class="material-symbols-outlined text-[14px]">box</span>' + esc(inst.mc_version || '—') + (inst.loader ? ' • ' + esc(inst.loader) : '') + '</div></div></div>';
          row.addEventListener('click', function () { selectInstance(inst.id); });
          itemsWrapper.appendChild(row);
        });
      }
      groupSection.appendChild(itemsWrapper);
      container.appendChild(groupSection);
    });

    if (selectedInstanceId && instanceDataMap[selectedInstanceId]) {
      updateDetailPanel(selectedInstanceId);
    }
  }

  function updateDetailPanel(id) {
    var panel = document.getElementById('detail-panel');
    if (!panel) return;
    panel.classList.remove('hidden'); panel.classList.add('flex');
    var inst = instanceDataMap[id];
    if (inst) {
      document.getElementById('detail-name').textContent = inst.name || '—';
      document.getElementById('detail-version').textContent = inst.mc_version || '—';
      document.getElementById('detail-loader').textContent = inst.loader || 'Vanilla';
      
      var iconBox = document.getElementById('detail-icon-box');
      if (iconBox) {
        if (inst.icon) {
          iconBox.innerHTML = '<img src="' + esc(inst.icon) + '" class="w-full h-full object-cover rounded-2xl"/>';
        } else {
          iconBox.innerHTML = '<span class="material-symbols-outlined text-[32px] text-text-secondary" id="detail-icon">deployed_code</span>';
        }
      }

      var pathEl = document.getElementById('detail-path');
      var fullPath = '%APPDATA%\\MyL\\instances\\' + inst.id;
      if (pathEl) { pathEl.textContent = fullPath; pathEl.title = fullPath; }
    }
    updateLaunchButtonState();
  }

  function selectInstance(id) {
    selectedInstanceId = id;
    localStorage.setItem('myl_last_instance', id);
    document.querySelectorAll('[data-id]').forEach(function (c) {
      var isSel = c.dataset.id === id;
      c.classList.toggle('border-primary', isSel);
      c.classList.toggle('ring-2', isSel);
      c.classList.toggle('ring-primary/20', isSel);
      c.classList.toggle('border-outline-variant', !isSel);
    });
    updateDetailPanel(id);
  }

  // === Game Launch & Process Stopping ===
  var btnLaunch = document.getElementById('btn-launch');

  function updateLaunchButtonState() {
    var content = document.getElementById('btn-launch-content');
    var statusText = document.getElementById('detail-status');
    var statusBox = document.getElementById('detail-status-container');
    if (runningInstancePid && runningInstanceId === selectedInstanceId) {
      btnLaunch.className = 'w-full bg-status-error text-white rounded-lg flex items-center overflow-hidden hover:brightness-110 active:scale-[0.98] transition-all group';
      var stopTxt = window.i18n ? window.i18n.t('btn_stop', 'Закрыть') : 'Закрыть';
      var inGameTxt = window.i18n ? window.i18n.t('in_game', 'В игре') : 'В игре';
      content.innerHTML = '<span class="material-symbols-outlined" style="font-variation-settings:\'FILL\' 1">stop</span><span class="font-label-md text-label-md font-bold">' + stopTxt + '</span>';
      if (statusText) statusText.textContent = inGameTxt + ' (PID: ' + runningInstancePid + ')';
      if (statusBox) statusBox.className = 'inline-block mt-2 px-3 py-1 bg-status-error/10 border border-status-error/30 rounded-full text-status-error';
    } else {
      btnLaunch.className = 'w-full bg-primary-container text-white rounded-lg flex items-center overflow-hidden hover:brightness-110 active:scale-[0.98] transition-all group';
      var launchTxt = window.i18n ? window.i18n.t('btn_launch', 'Запустить') : 'Запустить';
      var readyTxt = window.i18n ? window.i18n.t('ready_to_play', 'Готов к запуску') : 'Готов к запуску';
      content.innerHTML = '<span class="material-symbols-outlined" style="font-variation-settings:\'FILL\' 1">play_arrow</span><span class="font-label-md text-label-md font-bold">' + launchTxt + '</span>';
      if (statusText) statusText.textContent = readyTxt;
      if (statusBox) statusBox.className = 'inline-block mt-2 px-3 py-1 bg-status-success/10 border border-status-success/30 rounded-full text-status-success';
    }
  }

  btnLaunch.addEventListener('click', function () {
    if (!selectedInstanceId) { showAlert('Ошибка', 'Выберите сборку', false); return; }

    // If game is running, clicking "Закрыть" stops the game PID
    if (runningInstancePid && runningInstanceId === selectedInstanceId) {
      window.backend.stopGameByPid(runningInstancePid).then(function () {
        runningInstancePid = null;
        runningInstanceId = null;
        updateLaunchButtonState();
        showAlert('Игра остановлена', 'Процесс игры завершен', false);
      }).catch(function (e) {
        showAlert('Ошибка', String(e), false);
      });
      return;
    }

    var inst = instanceDataMap[selectedInstanceId];
    updateDownloadProgress(5, 'Подготовка запуска игры...');
    window.backend.ensureJava(inst ? inst.mc_version : '').then(function () {
      updateDownloadProgress(15, 'Проверка Java...');
      return window.backend.listAccounts();
    }).then(function (json) {
      var accs = JSON.parse(json);
      var acc = accs.find(function (a) { return a.account_type === 'offline'; });
      if (!acc) {
        updateDownloadProgress(-1);
        showAlert('Нет аккаунта', 'Создайте офлайн-аккаунт', false, 'info');
        return;
      }
      updateDownloadProgress(30, 'Подготовка библиотек и запуск...');
      return window.backend.launchInstance(selectedInstanceId, acc.id);
    }).then(function (resJson) {
      if (!resJson) return;
      var res = JSON.parse(resJson);
      if (res.pid) {
        runningInstancePid = res.pid;
        runningInstanceId = selectedInstanceId;
        updateLaunchButtonState();
        updateDownloadProgress(100, 'Игра успешно запущена!');
      }

      // Check launcher action setting upon launch
      var action = localStorage.getItem('myl_launch_action') || 'minimize';
      if (action === 'minimize') {
        window.windowControls.minimize();
      } else if (action === 'close') {
        window.windowControls.close();
      }
    }).catch(function (err) {
      updateDownloadProgress(-1);
      showAlert('Ошибка запуска', String(err), false, 'error');
    });
  });

  document.getElementById('btn-open-folder').addEventListener('click', function () {
    if (!selectedInstanceId) return;
    window.backend.openFolder('%APPDATA%\\MyL\\instances\\' + selectedInstanceId).catch(function (e) { showAlert('Ошибка', String(e), false); });
  });

  document.getElementById('btn-delete-instance').addEventListener('click', function () {
    if (!selectedInstanceId) return;
    var name = instanceDataMap[selectedInstanceId] ? instanceDataMap[selectedInstanceId].name : selectedInstanceId;
    showAlert('Удалить экземпляр?', 'Вы действительно хотите удалить сборку "' + name + '"? Это действие необратимо.', true).then(function (ok) {
      if (ok) window.backend.deleteInstance(selectedInstanceId).then(function () {
        document.getElementById('detail-panel').classList.add('hidden');
        selectedInstanceId = null; loadInstances();
      });
    });
  });

  // === Instance Settings Modal & Editing ===
  document.getElementById('btn-edit-instance').addEventListener('click', function () {
    if (!selectedInstanceId) return;
    var inst = instanceDataMap[selectedInstanceId];
    if (inst) {
      document.getElementById('settings-name').value = inst.name || '';
      document.getElementById('settings-group-input').value = inst.group || '';
      window.backend.getInstanceConfig(selectedInstanceId).then(function (json) {
        var cfg = JSON.parse(json);
        var defaultJvm = localStorage.getItem('myl_default_jvm_args') || '-XX:+UseG1GC -XX:+UnlockExperimentalVMOptions';
        document.getElementById('settings-jvm-args').value = cfg.jvm_args || defaultJvm;
        document.getElementById('settings-mc-version').value = cfg.mc_version || inst.mc_version || '';
        document.getElementById('settings-loader').value = cfg.loader || inst.loader || '';
        document.getElementById('settings-min-ram').value = cfg.min_memory_mb || 1024;
        document.getElementById('settings-max-ram').value = cfg.max_memory_mb || 4096;
        document.getElementById('settings-game-width').value = cfg.game_width || 854;
        document.getElementById('settings-game-height').value = cfg.game_height || 480;
        document.getElementById('settings-fullscreen').checked = !!cfg.fullscreen;
      }).catch(function () {});
    }
    openModal('modal-instance-settings');
    loadInstalledMods(selectedInstanceId);
  });

  document.getElementById('btn-settings-close').addEventListener('click', function () { closeModal('modal-instance-settings'); });
  document.getElementById('btn-settings-cancel').addEventListener('click', function () { closeModal('modal-instance-settings'); });

  var btnSettingsIcon = document.getElementById('btn-settings-icon');
  if (btnSettingsIcon) {
    btnSettingsIcon.addEventListener('click', function () {
      if (!selectedInstanceId) return;
      window.backend.setInstanceIcon(selectedInstanceId).then(function(newIcon) {
        if (newIcon) {
          if (instanceDataMap[selectedInstanceId]) {
            instanceDataMap[selectedInstanceId].icon = newIcon;
          }
          renderInstancesUI();
          showAlert('Иконка изменена', 'Иконка сборки успешно обновлена', false, 'success');
        }
      }).catch(function(e) {
        if (String(e) !== 'No file selected') {
          showAlert('Ошибка', String(e), false, 'error');
        }
      });
    });
  }

  document.getElementById('btn-settings-save').addEventListener('click', function () {
    if (!selectedInstanceId) { closeModal('modal-instance-settings'); return; }
    var newName = document.getElementById('settings-name').value.trim();
    var newGroup = document.getElementById('settings-group-input').value.trim();
    var newJvm = document.getElementById('settings-jvm-args').value.trim();
    var newMcVersion = document.getElementById('settings-mc-version').value.trim();
    var newLoader = document.getElementById('settings-loader').value.trim();
    var minRam = document.getElementById('settings-min-ram').value.trim();
    var maxRam = document.getElementById('settings-max-ram').value.trim();
    var gameWidth = document.getElementById('settings-game-width').value.trim();
    var gameHeight = document.getElementById('settings-game-height').value.trim();
    var fullscreen = document.getElementById('settings-fullscreen').checked;

    window.backend.updateInstanceConfig({
      id: selectedInstanceId,
      name: newName || undefined,
      group: newGroup ? newGroup : null,
      jvm_args: newJvm ? newJvm : null,
      mc_version: newMcVersion || undefined,
      loader: newLoader ? newLoader : null,
      min_memory_mb: minRam ? parseInt(minRam, 10) : null,
      max_memory_mb: maxRam ? parseInt(maxRam, 10) : null,
      game_width: gameWidth ? parseInt(gameWidth, 10) : null,
      game_height: gameHeight ? parseInt(gameHeight, 10) : null,
      fullscreen: fullscreen
    }).then(function () {
      closeModal('modal-instance-settings');
      loadInstances();
      showAlert('Сохранено', 'Настройки сборки сохранены', false);
    }).catch(function (e) {
      showAlert('Ошибка', String(e), false);
    });
  });

  // Settings tabs inside Instance Settings Modal
  document.querySelectorAll('.settings-tab').forEach(function (tab) {
    tab.addEventListener('click', function () {
      document.querySelectorAll('.settings-tab').forEach(function (t) { t.classList.remove('sidebar-item-active'); t.classList.add('text-on-surface-variant'); });
      tab.classList.add('sidebar-item-active'); tab.classList.remove('text-on-surface-variant');
      document.querySelectorAll('.settings-content').forEach(function (c) { c.classList.add('hidden'); });
      var target = document.getElementById('settings-tab-' + tab.dataset.tab);
      if (target) target.classList.remove('hidden');

      if (tab.dataset.tab === 'mods' && selectedInstanceId) {
        loadInstalledMods(selectedInstanceId);
      }
      if (tab.dataset.tab === 'resourcepacks' && selectedInstanceId) {
        loadResourcePacks(selectedInstanceId);
      }
      if (tab.dataset.tab === 'shaders' && selectedInstanceId) {
        loadShaderPacks(selectedInstanceId);
      }
      if (tab.dataset.tab === 'worlds' && selectedInstanceId) {
        loadWorlds(selectedInstanceId);
      }
      if (tab.dataset.tab === 'servers' && selectedInstanceId) {
        loadServers(selectedInstanceId);
      }
    });
  });

  // === Mods Management in Instance Settings ===
  var installedModsTbody = document.getElementById('installed-mods-tbody');
  var instanceModsListView = document.getElementById('instance-mods-list-view');
  var instanceModsSearchView = document.getElementById('instance-mods-search-view');
  var btnOpenModSearch = document.getElementById('btn-open-mod-search');
  var btnBackInstalledMods = document.getElementById('btn-back-installed-mods');
  var btnAddModFile = document.getElementById('btn-add-mod-file');
  var btnOpenModsFolder = document.getElementById('btn-open-mods-folder');

  function loadInstalledMods(instanceId) {
    if (!installedModsTbody) return;
    installedModsTbody.innerHTML = '<tr><td colspan="3" class="text-center py-8 text-text-secondary">Загрузка модов...</td></tr>';
    window.backend.listInstalledMods(instanceId).then(function (json) {
      var mods = JSON.parse(json) || [];
      document.getElementById('mods-count-installed').textContent = mods.length;
      document.getElementById('mods-count-active').textContent = mods.filter(function (m) { return m.enabled; }).length;

      installedModsTbody.innerHTML = '';
      if (!mods.length) {
        installedModsTbody.innerHTML = '<tr><td colspan="3" class="text-center py-8 text-text-secondary">Нет установленных модов</td></tr>';
        return;
      }

      mods.forEach(function (m) {
        var row = document.createElement('tr');
        row.className = 'hover:bg-surface-variant/20 transition-colors ' + (m.enabled ? '' : 'opacity-60 grayscale');
        var sizeMb = (m.size_bytes / (1024 * 1024)).toFixed(2) + ' MB';
        var modIconHtml = m.icon_base64 ? '<img src="' + esc(m.icon_base64) + '" class="w-8 h-8 rounded"/>' : '<span class="material-symbols-outlined text-primary text-[32px]">extension</span>';
        row.innerHTML =
          '<td class="px-4 py-3"><div class="flex items-center gap-3">' + modIconHtml +
          '<div><div class="font-body-md font-bold text-on-surface">' + esc(m.name) + '</div><div class="text-[11px] text-text-secondary">' + esc(m.filename) + ' • ' + sizeMb + '</div></div></div></td>' +
          '<td class="px-4 py-3 text-center">' +
          '<label class="relative inline-flex items-center cursor-pointer">' +
          '<input type="checkbox" class="sr-only peer mod-toggle-cb" data-filename="' + esc(m.filename) + '" ' + (m.enabled ? 'checked' : '') + '/>' +
          '<div class="w-9 h-5 bg-surface-variant peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-primary-container after:content-[\'\'] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"></div>' +
          '</label></td>' +
          '<td class="px-4 py-3 text-right"><button class="btn-del-mod p-1 text-status-error hover:bg-status-error/10 rounded transition-colors" data-filename="' + esc(m.filename) + '" title="Удалить"><span class="material-symbols-outlined text-[20px]">delete</span></button></td>';

        var toggleCb = row.querySelector('.mod-toggle-cb');
        toggleCb.addEventListener('change', function () {
          window.backend.toggleMod(instanceId, m.filename, toggleCb.checked).then(function () {
            loadInstalledMods(instanceId);
          });
        });

        var delBtn = row.querySelector('.btn-del-mod');
        delBtn.addEventListener('click', function () {
          window.backend.deleteMod(instanceId, m.filename).then(function () {
            loadInstalledMods(instanceId);
          });
        });

        installedModsTbody.appendChild(row);
      });
    }).catch(function (e) {
      installedModsTbody.innerHTML = '<tr><td colspan="3" class="text-center py-8 text-status-error">' + esc(String(e)) + '</td></tr>';
    });
  }

  if (btnOpenModSearch) {
    btnOpenModSearch.addEventListener('click', function () {
      instanceModsListView.classList.add('hidden');
      instanceModsSearchView.classList.remove('hidden');
      var inst = instanceDataMap[selectedInstanceId];
      if (inst) {
        searchInstanceMods('', inst.mc_version, inst.loader);
      }
    });
  }

  if (btnBackInstalledMods) {
    btnBackInstalledMods.addEventListener('click', function () {
      instanceModsSearchView.classList.add('hidden');
      instanceModsListView.classList.remove('hidden');
      if (selectedInstanceId) loadInstalledMods(selectedInstanceId);
    });
  }

  if (btnOpenModsFolder) {
    btnOpenModsFolder.addEventListener('click', function () {
      if (selectedInstanceId) {
        window.backend.openFolder('%APPDATA%\\MyL\\instances\\' + selectedInstanceId + '\\.minecraft\\mods').catch(function (e) { showAlert('Ошибка', String(e), false); });
      }
    });
  }

  if (btnAddModFile) {
    btnAddModFile.addEventListener('click', function () {
      var fileInput = document.createElement('input');
      fileInput.type = 'file';
      fileInput.accept = '.jar';
      fileInput.style.display = 'none';
      document.body.appendChild(fileInput);
      fileInput.addEventListener('change', function () {
        if (fileInput.files && fileInput.files.length > 0) {
          var srcPath = fileInput.files[0].path || fileInput.files[0].name;
          window.backend.addModFile(selectedInstanceId, srcPath).then(function () {
            loadInstalledMods(selectedInstanceId);
          }).catch(function (e) { showAlert('Ошибка', String(e), false); });
        }
        document.body.removeChild(fileInput);
      });
      fileInput.click();
    });
  }

  var instanceModSearchInput = document.getElementById('instance-mod-search-input');
  var instanceModSearchResults = document.getElementById('instance-mod-search-results');
  var modDetailDrawer = document.getElementById('mod-detail-drawer');
  var selectedModForDrawer = null;

  if (instanceModSearchInput) {
    instanceModSearchInput.addEventListener('keydown', function (e) {
      if (e.key === 'Enter') {
        var inst = instanceDataMap[selectedInstanceId];
        searchInstanceMods(instanceModSearchInput.value.trim(), inst ? inst.mc_version : '', inst ? inst.loader : '');
      }
    });
  }

  function searchInstanceMods(q, mcVer, loader) {
    if (!instanceModSearchResults) return;
    instanceModSearchResults.innerHTML = '<div class="text-center py-12 text-text-secondary"><div class="animate-spin w-8 h-8 border-2 border-primary border-t-transparent rounded-full mx-auto mb-4"></div><p>Поиск модов...</p></div>';
    window.backend.searchMods(q, mcVer || '', loader || '', 0).then(function (json) {
      var mods = JSON.parse(json) || [];
      instanceModSearchResults.innerHTML = '';
      if (!mods.length) {
        instanceModSearchResults.innerHTML = '<div class="text-center py-12 text-text-secondary">Ничего не найдено</div>';
        return;
      }
      mods.forEach(function (m) {
        var card = document.createElement('div');
        card.className = 'flex items-center gap-3 p-3 bg-surface-card rounded-xl border border-transparent hover:border-primary/40 transition-all cursor-pointer';
        var icon = m.icon_url ? '<img src="' + esc(m.icon_url) + '" class="w-10 h-10 rounded-lg object-cover bg-surface-container-high shrink-0"/>' : '<div class="w-10 h-10 rounded-lg bg-surface-container-high flex items-center justify-center shrink-0"><span class="material-symbols-outlined text-text-secondary text-sm">extension</span></div>';
        card.innerHTML = icon + '<div class="flex-1 min-w-0"><h4 class="font-body-md font-bold text-on-surface truncate">' + esc(m.title) + '</h4><p class="text-label-sm text-text-secondary truncate">' + esc(m.description) + '</p></div>' +
          '<button class="btn-inst-mod px-3 py-1.5 bg-primary-container text-white text-label-md font-bold rounded-lg hover:brightness-110 shrink-0" data-slug="' + esc(m.id) + '" data-title="' + esc(m.title) + '">Скачать</button>';

        // Click card opens drawer on the left
        card.addEventListener('click', function (ev) {
          if (ev.target.classList.contains('btn-inst-mod')) return;
          openModDrawer(m);
        });

        card.querySelector('.btn-inst-mod').addEventListener('click', function (ev) {
          ev.stopPropagation();
          openModDrawer(m);
        });

        instanceModSearchResults.appendChild(card);
      });
    }).catch(function (e) {
      instanceModSearchResults.innerHTML = '<div class="text-center py-12 text-status-error">' + esc(String(e)) + '</div>';
    });
  }

  function openModDrawer(mod) {
    if (!modDetailDrawer) return;
    selectedModForDrawer = mod;
    document.getElementById('drawer-mod-title').textContent = mod.title;
    document.getElementById('drawer-mod-author').textContent = 'Автор: ' + (mod.author || '—');
    document.getElementById('drawer-mod-desc').textContent = mod.description || 'Описание отсутствует';
    var iconEl = document.getElementById('drawer-mod-icon');
    if (mod.icon_url) { iconEl.src = mod.icon_url; iconEl.classList.remove('hidden'); } else { iconEl.classList.add('hidden'); }
    modDetailDrawer.classList.remove('hidden'); modDetailDrawer.classList.add('flex');

    // Load versions from Modrinth for the selected mod
    var versionSelect = document.getElementById('drawer-version-select');
    var versionMeta = document.getElementById('drawer-version-meta');
    var versionsSpinner = document.getElementById('drawer-versions-spinner');
    if (versionSelect) {
      versionSelect.innerHTML = '<option value="">Загрузка версий...</option>';
      versionSelect.disabled = true;
      if (versionMeta) versionMeta.textContent = '';
      if (versionsSpinner) versionsSpinner.classList.remove('hidden');

      var inst = instanceDataMap[selectedInstanceId];
      var mcVer = inst ? inst.mc_version : '';
      var loader = inst ? (inst.loader || '') : '';

      window.backend.getModVersions(mod.id, mcVer, loader).then(function (json) {
        var versions = JSON.parse(json) || [];
        if (versionsSpinner) versionsSpinner.classList.add('hidden');
        if (!versions.length) {
          versionSelect.innerHTML = '<option value="">Нет совместимых версий</option>';
          versionSelect.disabled = true;
          if (versionMeta) versionMeta.textContent = mcVer ? 'Нет версий для Minecraft ' + mcVer + (loader ? ' (' + loader + ')' : '') : 'Нет доступных версий';
          return;
        }
        versionSelect.innerHTML = '';
        versionSelect.disabled = false;
        versions.forEach(function (v, idx) {
          var opt = document.createElement('option');
          opt.value = idx;
          var label = v.name || v.version;
          var mcList = v.mc_versions && v.mc_versions.length ? ' [' + v.mc_versions.slice(0, 2).join(', ') + (v.mc_versions.length > 2 ? '…' : '') + ']' : '';
          opt.textContent = label + mcList;
          opt.dataset.idx = idx;
          versionSelect.appendChild(opt);
        });
        // Store versions array for later use by install button
        versionSelect.dataset.versionsJson = JSON.stringify(versions);
        // Show meta for first version
        updateDrawerVersionMeta(versions[0], versionMeta);
        versionSelect.onchange = function () {
          var idx = parseInt(versionSelect.value, 10);
          if (!isNaN(idx) && versions[idx]) updateDrawerVersionMeta(versions[idx], versionMeta);
        };
      }).catch(function (e) {
        if (versionsSpinner) versionsSpinner.classList.add('hidden');
        versionSelect.innerHTML = '<option value="">Ошибка загрузки</option>';
        if (versionMeta) versionMeta.textContent = String(e);
      });
    }
  }

  function updateDrawerVersionMeta(v, metaEl) {
    if (!metaEl || !v) return;
    var parts = [];
    if (v.mc_versions && v.mc_versions.length) parts.push('MC: ' + v.mc_versions.join(', '));
    if (v.loaders && v.loaders.length) parts.push('Загрузчик: ' + v.loaders.join(', '));
    if (v.size) parts.push('Размер: ' + (v.size / (1024 * 1024)).toFixed(2) + ' MB');
    metaEl.textContent = parts.join(' • ');
  }

  document.getElementById('btn-close-mod-drawer').addEventListener('click', function () {
    if (modDetailDrawer) {
      modDetailDrawer.classList.add('hidden');
      modDetailDrawer.classList.remove('flex');
      // Reset version dropdown state
      var sel = document.getElementById('drawer-version-select');
      if (sel) { sel.innerHTML = '<option value="">\u0417\u0430\u0433\u0440\u0443\u0437\u043a\u0430...</option>'; sel.disabled = true; sel.dataset.versionsJson = ''; }
      var meta = document.getElementById('drawer-version-meta');
      if (meta) meta.textContent = '';
    }
  });

  document.getElementById('btn-drawer-install').addEventListener('click', function () {
    if (!selectedModForDrawer) return;
    var versionSelect = document.getElementById('drawer-version-select');
    if (versionSelect && versionSelect.dataset.versionsJson && !versionSelect.disabled) {
      var versions = JSON.parse(versionSelect.dataset.versionsJson);
      var idx = parseInt(versionSelect.value, 10);
      var chosenVersion = (!isNaN(idx) && versions[idx]) ? versions[idx] : (versions[0] || null);
      if (chosenVersion) {
        installModToInstanceWithVersion(selectedModForDrawer.id, selectedModForDrawer.title, chosenVersion);
        return;
      }
    }
    // Fallback: auto-pick first compatible version
    installModToInstance(selectedModForDrawer.id, selectedModForDrawer.title);
  });

  function installModToInstance(slug, title, targetId) {
    var instId = targetId || selectedInstanceId;
    var inst = instanceDataMap[instId];
    if (!inst) {
      showAlert('Выберите сборку', 'Сначала выберите сборку для установки мода', false, 'info');
      return;
    }
    var mcVer = inst.mc_version || '';
    var loader = inst.loader || '';
    showAlert('Поиск версий', 'Проверка совместимости версий для ' + esc(inst.name) + ' (' + mcVer + ')...', false, 'info');

    window.backend.getModVersions(slug, mcVer, loader).then(function (json) {
      var vers = JSON.parse(json) || [];
      if (!vers.length) {
        showAlert('Несовместимость', 'Мод "' + title + '" не поддерживает версию Minecraft ' + mcVer + (loader ? ' (' + loader + ')' : ''), false, 'error');
        return;
      }
      var v = vers[0];
      if (!v.download_url) {
        showAlert('Ошибка', 'У данной версии мода нет ссылки для скачивания', false, 'error');
        return;
      }
      var relType = v.version_type ? (v.version_type.charAt(0).toUpperCase() + v.version_type.slice(1)) : 'Release';
      var verLabel = (v.version_number || mcVer) + ' (' + relType + ')';

      showAlert('Загрузка', 'Скачивание "' + title + '" ' + verLabel + '...', false, 'info');
      window.backend.downloadMod(v.download_url, instId, v.filename).then(function () {
        showAlert('Успех', '"' + title + '" ' + verLabel + ' успешно установлен в сборку "' + inst.name + '"!', false, 'success');
        if (instId === selectedInstanceId) {
          loadInstalledMods(instId);
        }
      }).catch(function (e) {
        showAlert('Ошибка', String(e), false, 'error');
      });
    }).catch(function (e) {
      showAlert('Ошибка', String(e), false, 'error');
    });
  }

  // Install a mod using a pre-selected version object (from the version dropdown)
  function installModToInstanceWithVersion(slug, title, chosenVersion) {
    var instId = selectedInstanceId;
    var inst = instanceDataMap[instId];
    if (!inst) {
      showAlert('Выберите сборку', 'Сначала выберите сборку для установки мода', false, 'info');
      return;
    }
    if (!chosenVersion.download_url) {
      showAlert('Ошибка', 'У выбранной версии мода нет ссылки для скачивания', false, 'error');
      return;
    }
    var verLabel = chosenVersion.name || chosenVersion.version || '';
    showAlert('Загрузка', 'Скачивание «' + title + '» ' + verLabel + '...', false, 'info');
    window.backend.downloadMod(chosenVersion.download_url, instId, chosenVersion.filename).then(function () {
      showAlert('Успех', '«' + title + '» ' + verLabel + ' успешно установлен в сборку «' + inst.name + '»!', false, 'success');
      // Close drawer and refresh mods list
      if (modDetailDrawer) { modDetailDrawer.classList.add('hidden'); modDetailDrawer.classList.remove('flex'); }
      loadInstalledMods(instId);
    }).catch(function (e) {
      showAlert('Ошибка', String(e), false, 'error');
    });
  }

  // === Launcher Settings (Themes, Typography, Density, Animations) ===
  function initLauncherSettings() {
    var savedLang = window.i18n ? window.i18n.getCurrentLang() : 'ru';
    if (window.i18n) window.i18n.applyLanguage(savedLang);
    document.querySelectorAll('.lang-btn').forEach(function (btn) {
      var isSel = btn.dataset.lang === savedLang;
      btn.className = isSel ? 'lang-btn flex items-center justify-center gap-3 py-3 px-4 bg-primary/10 text-primary border-2 border-primary rounded-xl font-bold transition-all shadow-sm cursor-pointer' : 'lang-btn flex items-center justify-center gap-3 py-3 px-4 bg-surface-card hover:bg-surface-variant text-on-surface border border-outline-variant rounded-xl font-bold transition-all cursor-pointer';
      btn.onclick = function () {
        if (window.i18n) {
          window.i18n.applyLanguage(btn.dataset.lang);
          initLauncherSettings();
          renderInstancesUI();
          loadAccounts();
          updateLaunchButtonState();
          if (selectedInstanceId) updateDetailPanel(selectedInstanceId);
        }
      };
    });

    var savedTheme = localStorage.getItem('myl_theme') || 'dark';
    applyTheme(savedTheme);
    document.querySelectorAll('.theme-card').forEach(function (card) {
      var isSel = card.dataset.theme === savedTheme;
      card.classList.toggle('border-2', isSel);
      card.classList.toggle('border-primary', isSel);
      card.classList.toggle('shadow-lg', isSel);
      card.classList.toggle('border-outline-variant', !isSel);
      card.onclick = function () {
        applyTheme(card.dataset.theme);
        initLauncherSettings();
      };
    });

    var savedFontSize = localStorage.getItem('myl_font_size') || '14';
    var fontSlider = document.getElementById('font-size-slider');
    var fontVal = document.getElementById('font-size-val');
    var fontRaf = null;
    var lastAppliedFontVal = null;

    if (fontSlider && fontVal) {
      fontSlider.value = savedFontSize;
      var numVal = parseFloat(savedFontSize) || 14;
      fontVal.textContent = (numVal % 1 === 0 ? numVal.toFixed(0) : numVal.toFixed(1)) + 'px';
      document.documentElement.style.fontSize = (numVal / 14 * 100) + '%';
      lastAppliedFontVal = numVal;

      fontSlider.oninput = function () {
        var v = parseFloat(fontSlider.value);
        fontVal.textContent = (v % 1 === 0 ? v.toFixed(0) : v.toFixed(1)) + 'px';
        
        if (lastAppliedFontVal === null || Math.abs(v - lastAppliedFontVal) >= 0.2) {
          lastAppliedFontVal = v;
          if (fontRaf) cancelAnimationFrame(fontRaf);
          fontRaf = requestAnimationFrame(function () {
            document.documentElement.style.fontSize = (v / 14 * 100) + '%';
          });
        }
      };
      fontSlider.onchange = function () {
        var v = parseFloat(fontSlider.value);
        lastAppliedFontVal = v;
        document.documentElement.style.fontSize = (v / 14 * 100) + '%';
        localStorage.setItem('myl_font_size', fontSlider.value);
      };
    }

    var savedDensity = localStorage.getItem('myl_density') || 'normal';
    applyDensity(savedDensity);
    document.querySelectorAll('.density-btn').forEach(function (btn) {
      var isSel = btn.dataset.density === savedDensity;
      btn.className = isSel ? 'density-btn flex-1 py-2 bg-primary/10 text-primary rounded-lg font-label-md text-label-md font-bold border border-primary/30 shadow-sm' : 'density-btn flex-1 py-2 bg-surface-variant text-on-surface rounded-lg font-label-md text-label-md font-bold border border-outline-variant hover:bg-surface-bright transition-colors';
      btn.onclick = function () {
        applyDensity(btn.dataset.density);
        initLauncherSettings();
      };
    });

    var savedAnim = localStorage.getItem('myl_animations') !== 'false';
    var toggleAnim = document.getElementById('toggle-animations');
    if (toggleAnim) {
      toggleAnim.checked = savedAnim;
      applyAnimations(savedAnim);
      toggleAnim.onchange = function () {
        applyAnimations(toggleAnim.checked);
      };
    }

    var savedAction = localStorage.getItem('myl_launch_action') || 'minimize';
    var globalActionSelect = document.getElementById('global-window-action');
    if (globalActionSelect) globalActionSelect.value = savedAction;

    var savedJvm = localStorage.getItem('myl_default_jvm_args') || '-Xmx2G -XX:+UnlockExperimentalVMOptions -XX:+UseG1GC -XX:G1NewSizePercent=20 -XX:G1ReservePercent=20 -XX:MaxGCPauseMillis=50 -XX:G1HeapRegionSize=32M';
    var globalJvmArea = document.getElementById('global-default-jvm-args');
    if (globalJvmArea) globalJvmArea.value = savedJvm;

    var btnSaveSettings = document.getElementById('btn-save-settings');
    if (btnSaveSettings) {
      btnSaveSettings.onclick = function () {
        if (globalActionSelect) localStorage.setItem('myl_launch_action', globalActionSelect.value);
        if (globalJvmArea) localStorage.setItem('myl_default_jvm_args', globalJvmArea.value.trim());
        showAlert('Успех', 'Общие настройки сохранены', false, 'success');
      };
    }
  }

  function applyTheme(theme) {
    document.documentElement.classList.remove('theme-dark', 'theme-onyx', 'theme-light', 'theme-ember');
    if (theme === 'onyx') document.documentElement.classList.add('theme-onyx');
    else if (theme === 'light') document.documentElement.classList.add('theme-light');
    else if (theme === 'ember') document.documentElement.classList.add('theme-ember');
    else document.documentElement.classList.add('theme-dark');
    localStorage.setItem('myl_theme', theme);
  }

  function applyDensity(density) {
    document.body.classList.remove('density-compact', 'density-spacious');
    if (density === 'compact') document.body.classList.add('density-compact');
    else if (density === 'spacious') document.body.classList.add('density-spacious');
    localStorage.setItem('myl_density', density);
  }

  function applyAnimations(enabled) {
    if (!enabled) document.documentElement.classList.add('no-animations');
    else document.documentElement.classList.remove('no-animations');
    localStorage.setItem('myl_animations', enabled ? 'true' : 'false');
  }

  // === ResourcePacks, ShaderPacks, Worlds, Servers Loaders ===
  function loadResourcePacks(instanceId) {
    var container = document.getElementById('rp-list-container');
    if (!container) return;
    var loadingTxt = window.i18n ? window.i18n.t('loading_rp', 'Загрузка ресурс-паков...') : 'Загрузка ресурс-паков...';
    var noRpTxt = window.i18n ? window.i18n.t('no_rp', 'Нет установленных ресурс-паков') : 'Нет установленных ресурс-паков';
    container.innerHTML = '<div class="text-center py-8 text-text-secondary">' + loadingTxt + '</div>';
    window.backend.listResourcepacks(instanceId).then(function(json) {
      var list = JSON.parse(json) || [];
      if (!list.length) {
        container.innerHTML = '<div class="text-center py-8 text-text-secondary">' + noRpTxt + '</div>';
        return;
      }
      container.innerHTML = '';
      list.forEach(function(rp) {
        var item = document.createElement('div');
        item.className = 'p-3 bg-surface-container-low border border-outline-variant/30 rounded-xl flex items-center gap-3';
        var iconHtml = rp.icon_base64 ? '<img src="' + esc(rp.icon_base64) + '" class="w-10 h-10 rounded-lg object-cover"/>' : '<div class="w-10 h-10 bg-surface-variant rounded-lg flex items-center justify-center shrink-0"><span class="material-symbols-outlined text-primary">palette</span></div>';
        item.innerHTML = iconHtml + '<div class="flex-1 overflow-hidden"><div class="font-bold text-on-surface text-body-md truncate">' + esc(rp.name) + '</div><div class="text-label-sm text-text-secondary truncate">' + esc(rp.description || rp.filename) + '</div></div>' +
          '<button class="btn-del-rp p-1.5 text-status-error hover:bg-status-error/10 rounded transition-colors" data-filename="' + esc(rp.filename) + '" title="Удалить"><span class="material-symbols-outlined text-[20px]">delete</span></button>';
        var delBtn = item.querySelector('.btn-del-rp');
        delBtn.addEventListener('click', function() {
          window.backend.deleteResourcepack(instanceId, rp.filename).then(function() {
            loadResourcePacks(instanceId);
          });
        });
        container.appendChild(item);
      });
    }).catch(function(e) {
      container.innerHTML = '<div class="text-center py-8 text-status-error">' + esc(String(e)) + '</div>';
    });
  }

  function loadShaderPacks(instanceId) {
    var container = document.getElementById('sp-list-container');
    if (!container) return;
    var loadingSpTxt = window.i18n ? window.i18n.t('loading_shaders', 'Загрузка шейдеров...') : 'Загрузка шейдеров...';
    var noSpTxt = window.i18n ? window.i18n.t('no_shaders', 'Нет установленных шейдеров') : 'Нет установленных шейдеров';
    container.innerHTML = '<div class="text-center py-8 text-text-secondary">' + loadingSpTxt + '</div>';
    window.backend.listShaderpacks(instanceId).then(function(json) {
      var list = JSON.parse(json) || [];
      if (!list.length) {
        container.innerHTML = '<div class="text-center py-8 text-text-secondary">' + noSpTxt + '</div>';
        return;
      }
      container.innerHTML = '';
      list.forEach(function(sp) {
        var item = document.createElement('div');
        item.className = 'p-3 bg-surface-container-low border border-outline-variant/30 rounded-xl flex items-center gap-3';
        item.innerHTML = '<div class="w-10 h-10 bg-surface-variant rounded-lg flex items-center justify-center shrink-0"><span class="material-symbols-outlined text-primary">blur_on</span></div><div class="flex-1 overflow-hidden"><div class="font-bold text-on-surface text-body-md truncate">' + esc(sp.name) + '</div></div>' +
          '<button class="btn-del-sp p-1.5 text-status-error hover:bg-status-error/10 rounded transition-colors" data-filename="' + esc(sp.filename) + '" title="Удалить"><span class="material-symbols-outlined text-[20px]">delete</span></button>';
        var delBtn = item.querySelector('.btn-del-sp');
        delBtn.addEventListener('click', function() {
          window.backend.deleteShaderpack(instanceId, sp.filename).then(function() {
            loadShaderPacks(instanceId);
          });
        });
        container.appendChild(item);
      });
    }).catch(function(e) {
      container.innerHTML = '<div class="text-center py-8 text-status-error">' + esc(String(e)) + '</div>';
    });
  }

  function loadWorlds(instanceId) {
    var container = document.getElementById('worlds-list-container');
    if (!container) return;
    var loadingWorldsTxt = window.i18n ? window.i18n.t('loading_worlds', 'Загрузка миров...') : 'Загрузка миров...';
    var noWorldsTxt = window.i18n ? window.i18n.t('no_worlds', 'Нет сохраненных миров') : 'Нет сохраненных миров';
    container.innerHTML = '<div class="text-center py-8 text-text-secondary">' + loadingWorldsTxt + '</div>';
    window.backend.listWorlds(instanceId).then(function(json) {
      var list = JSON.parse(json) || [];
      if (!list.length) {
        container.innerHTML = '<div class="text-center py-8 text-text-secondary">' + noWorldsTxt + '</div>';
        return;
      }
      container.innerHTML = '';
      list.forEach(function(w) {
        var item = document.createElement('div');
        item.className = 'p-3 bg-surface-container-low border border-outline-variant/30 rounded-xl flex items-center gap-3';
        var iconHtml = w.icon_base64 ? '<img src="' + esc(w.icon_base64) + '" class="w-10 h-10 rounded-lg object-cover"/>' : '<div class="w-10 h-10 bg-surface-variant rounded-lg flex items-center justify-center shrink-0"><span class="material-symbols-outlined text-primary">public</span></div>';
        item.innerHTML = iconHtml + '<div class="flex-1 overflow-hidden"><div class="font-bold text-on-surface text-body-md truncate">' + esc(w.name) + '</div><div class="text-label-sm text-text-secondary truncate">' + esc(w.folder_name) + '</div></div>' +
          '<button class="btn-del-world p-1.5 text-status-error hover:bg-status-error/10 rounded transition-colors" data-foldername="' + esc(w.folder_name) + '" title="Удалить"><span class="material-symbols-outlined text-[20px]">delete</span></button>';
        var delBtn = item.querySelector('.btn-del-world');
        delBtn.addEventListener('click', function() {
          window.backend.deleteWorld(instanceId, w.folder_name).then(function() {
            loadWorlds(instanceId);
          });
        });
        container.appendChild(item);
      });
    }).catch(function(e) {
      container.innerHTML = '<div class="text-center py-8 text-status-error">' + esc(String(e)) + '</div>';
    });
  }

  function loadServers(instanceId) {
    var container = document.getElementById('servers-list-container');
    if (!container) return;
    var loadingServersTxt = window.i18n ? window.i18n.t('loading_servers', 'Загрузка серверов...') : 'Загрузка серверов...';
    var noServersTxt = window.i18n ? window.i18n.t('no_servers', 'Нет сохраненных серверов') : 'Нет сохраненных серверов';
    container.innerHTML = '<div class="text-center py-8 text-text-secondary">' + loadingServersTxt + '</div>';
    window.backend.listServers(instanceId).then(function(json) {
      var list = JSON.parse(json) || [];
      if (!list.length) {
        container.innerHTML = '<div class="text-center py-8 text-text-secondary">' + noServersTxt + '</div>';
        return;
      }
      container.innerHTML = '';
      list.forEach(function(s) {
        var item = document.createElement('div');
        item.className = 'p-3 bg-surface-container-low border border-outline-variant/30 rounded-xl flex items-center gap-3';
        var iconHtml = s.icon_base64 ? '<img src="' + esc(s.icon_base64) + '" class="w-10 h-10 rounded-lg object-cover"/>' : '<div class="w-10 h-10 bg-surface-variant rounded-lg flex items-center justify-center shrink-0"><span class="material-symbols-outlined text-primary">dns</span></div>';
        item.innerHTML = iconHtml + '<div class="flex-1 overflow-hidden"><div class="font-bold text-on-surface text-body-md truncate">' + esc(s.name || 'Сервер') + '</div><div class="text-label-sm text-text-secondary truncate">' + esc(s.ip) + '</div></div>';
        container.appendChild(item);
      });
    }).catch(function(e) {
      container.innerHTML = '<div class="text-center py-8 text-status-error">' + esc(String(e)) + '</div>';
    });
  }

  var btnOpenRpFolder = document.getElementById('btn-open-rp-folder');
  if (btnOpenRpFolder) {
    btnOpenRpFolder.addEventListener('click', function() {
      if (selectedInstanceId) window.backend.openFolder('%APPDATA%\\MyL\\instances\\' + selectedInstanceId + '\\.minecraft\\resourcepacks');
    });
  }

  var btnOpenSpFolder = document.getElementById('btn-open-sp-folder');
  if (btnOpenSpFolder) {
    btnOpenSpFolder.addEventListener('click', function() {
      if (selectedInstanceId) window.backend.openFolder('%APPDATA%\\MyL\\instances\\' + selectedInstanceId + '\\.minecraft\\shaderpacks');
    });
  }

  var btnOpenWorldsFolder = document.getElementById('btn-open-worlds-folder');
  if (btnOpenWorldsFolder) {
    btnOpenWorldsFolder.addEventListener('click', function() {
      if (selectedInstanceId) window.backend.openFolder('%APPDATA%\\MyL\\instances\\' + selectedInstanceId + '\\.minecraft\\saves');
    });
  }

  var btnOpenServersFolder = document.getElementById('btn-open-servers-folder');
  if (btnOpenServersFolder) {
    btnOpenServersFolder.addEventListener('click', function() {
      if (selectedInstanceId) window.backend.openFolder('%APPDATA%\\MyL\\instances\\' + selectedInstanceId + '\\.minecraft');
    });
  }

  var btnImportWorldZip = document.getElementById('btn-import-world-zip');
  if (btnImportWorldZip) {
    btnImportWorldZip.addEventListener('click', function() {
      if (!selectedInstanceId) return;
      window.backend.importWorldZip(selectedInstanceId).then(function(name) {
        loadWorlds(selectedInstanceId);
        showAlert('Успех', 'Мир "' + name + '" успешно импортирован!', false, 'success');
      }).catch(function(e) {
        if (String(e) !== 'No file selected') showAlert('Ошибка', String(e), false, 'error');
      });
    });
  }

  // === Console Output & Game Log Event Handler ===
  var consoleOutput = document.getElementById('console-output');
  var consoleAutoscroll = document.getElementById('console-autoscroll');
  var btnClearConsole = document.getElementById('btn-clear-console');

  if (btnClearConsole && consoleOutput) {
    btnClearConsole.addEventListener('click', function () {
      consoleOutput.innerHTML = '<div class="text-text-secondary italic">Консоль очищена.</div>';
    });
  }

  function appendConsoleLog(msg) {
    if (!consoleOutput) return;
    var line = document.createElement('div');
    line.className = 'py-0.5 hover:bg-surface-variant/20 px-1 rounded transition-colors';
    line.textContent = msg;
    consoleOutput.appendChild(line);
    if (consoleAutoscroll && consoleAutoscroll.checked) {
      consoleOutput.scrollTop = consoleOutput.scrollHeight;
    }
  }

  if (window.backend && window.backend.onGameLog) {
    window.backend.onGameLog(function (msg) {
      appendConsoleLog(msg);
    });
  }

  if (window.backend && window.backend.onGameExit) {
    window.backend.onGameExit(function (data) {
      eprintln_log('Game process exited:', data);
      runningInstancePid = null;
      runningInstanceId = null;
      updateLaunchButtonState();
      updateDownloadProgress(-1);
    });
  }
  function eprintln_log(a, b) { try { console.log(a, b); } catch(e){} }

  // === Progress Bar Event Handler (MC/code.html card style) ===
  var progressBox = document.getElementById('download-progress-box');
  var progressInner = document.getElementById('download-progress-inner');
  var progressPercent = document.getElementById('download-progress-percent');
  var progressStatus = document.getElementById('download-progress-status');
  var progressSub = document.getElementById('download-progress-sub');

  function updateDownloadProgress(pct, statusText, subText) {
    if (!progressBox) return;
    if (pct < 0) {
      progressBox.classList.add('hidden');
      return;
    }
    progressBox.classList.remove('hidden');
    if (progressInner) progressInner.style.width = Math.min(100, Math.max(0, pct)) + '%';
    if (progressPercent) progressPercent.textContent = Math.round(pct) + '%';
    if (progressStatus && statusText) progressStatus.textContent = statusText;
    if (progressSub) progressSub.textContent = subText || '';

    if (pct >= 100) {
      setTimeout(function () {
        if (progressBox) progressBox.classList.add('hidden');
      }, 1500);
    }
  }
  window.updateDownloadProgress = updateDownloadProgress;

  if (window.backend && window.backend.onDownloadProgress) {
    window.backend.onDownloadProgress(function (data) {
      if (data) {
        var done = data.done || 0;
        var total = data.total || 1;
        var pct = Math.min(100, Math.round((done / total) * 100));
        var taskLabel = data.task_id === 'assets' ? 'Загрузка ресурсов' : data.task_id === 'libraries' ? 'Загрузка библиотек' : 'Загрузка компонентов';
        updateDownloadProgress(pct, taskLabel + ':', done + '/' + total + ' файлов');
      }
    });
  }

  // === Canvas Skin Head Renderer ===
  function drawHeadOnCanvas(canvas, skinDataUrl) {
    if (!canvas) return;
    var ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    if (!skinDataUrl) {
      // Crisp Offline Steve Head
      ctx.fillStyle = '#C68966';
      ctx.fillRect(0, 0, canvas.width, canvas.height);
      ctx.fillStyle = '#382216';
      ctx.fillRect(0, 0, canvas.width, canvas.height * 0.3);
      ctx.fillRect(0, canvas.height * 0.3, canvas.width * 0.125, canvas.height * 0.2);
      ctx.fillRect(canvas.width * 0.875, canvas.height * 0.3, canvas.width * 0.125, canvas.height * 0.2);
      ctx.fillStyle = '#FFFFFF';
      ctx.fillRect(canvas.width * 0.125, canvas.height * 0.5, canvas.width * 0.25, canvas.height * 0.125);
      ctx.fillRect(canvas.width * 0.625, canvas.height * 0.5, canvas.width * 0.25, canvas.height * 0.125);
      ctx.fillStyle = '#414D96';
      ctx.fillRect(canvas.width * 0.25, canvas.height * 0.5, canvas.width * 0.125, canvas.height * 0.125);
      ctx.fillRect(canvas.width * 0.625, canvas.height * 0.5, canvas.width * 0.125, canvas.height * 0.125);
      ctx.fillStyle = '#5A3826';
      ctx.fillRect(canvas.width * 0.375, canvas.height * 0.75, canvas.width * 0.25, canvas.height * 0.125);
      return;
    }

    var img = new Image();
    img.onload = function() {
      ctx.imageSmoothingEnabled = false;
      ctx.drawImage(img, 8, 8, 8, 8, 0, 0, canvas.width, canvas.height);
      ctx.drawImage(img, 40, 8, 8, 8, 0, 0, canvas.width, canvas.height);
    };
    img.onerror = function() {
      drawHeadOnCanvas(canvas, null);
    };
    img.src = skinDataUrl;
  }

  // === Accounts ===
  var sidebarAccWidget = document.getElementById('sidebar-active-account');
  if (sidebarAccWidget) {
    sidebarAccWidget.addEventListener('click', function() {
      var accTab = document.querySelector('[data-page="accounts"]');
      if (accTab) accTab.click();
    });
  }
  document.getElementById('btn-add-account').addEventListener('click', function () { openModal('modal-add-account'); });
  document.getElementById('add-account-placeholder').addEventListener('click', function () { openModal('modal-add-account'); });
  document.getElementById('btn-modal-account-close').addEventListener('click', function () { closeModal('modal-add-account'); });
  
  var msaPollTimer = null;
  var msaCompleted = false;

  document.getElementById('btn-add-msa').addEventListener('click', function () {
    closeModal('modal-add-account');
    msaCompleted = false;

    // 1. Open the Microsoft Authorization Modal immediately
    document.getElementById('msa-code').textContent = '...';
    document.getElementById('msa-status-text').textContent = 'Запрос кода Microsoft...';
    openModal('modal-msa');

    // 2. Start Device Code Flow (displays link & 8-digit code inside the modal)
    window.backend.startMsaAuth().then(function(json) {
      if (msaCompleted) return;
      var dc = JSON.parse(json);
      document.getElementById('msa-link').href = dc.verification_uri;
      document.getElementById('msa-link').textContent = dc.verification_uri;
      document.getElementById('msa-code').textContent = dc.user_code;
      document.getElementById('msa-status-text').textContent = 'Ожидание подтверждения в браузере или по коду...';

      if (msaPollTimer) clearInterval(msaPollTimer);
      var intervalMs = (dc.interval || 5) * 1000;
      msaPollTimer = setInterval(function() {
        if (msaCompleted) return;
        window.backend.pollMsaAuth(dc.device_code).then(function(resJson) {
          if (resJson && !msaCompleted) {
            msaCompleted = true;
            if (msaPollTimer) clearInterval(msaPollTimer);
            msaPollTimer = null;
            closeModal('modal-msa');
            loadAccounts();
            showAlert('Успех', 'Microsoft аккаунт успешно добавлен!', false, 'success');
          }
        }).catch(function(e) {
          if (String(e) !== 'PENDING' && !msaCompleted) {
            console.warn('Device code polling:', e);
          }
        });
      }, intervalMs);
    }).catch(function(e) {
      console.warn('Device code start error:', e);
    });

    // 3. Simultaneously launch Browser PKCE Flow
    window.backend.loginMsaPkce().then(function(resJson) {
      if (resJson && !msaCompleted) {
        msaCompleted = true;
        if (msaPollTimer) clearInterval(msaPollTimer);
        msaPollTimer = null;
        closeModal('modal-msa');
        loadAccounts();
        showAlert('Успех', 'Microsoft аккаунт успешно добавлен!', false, 'success');
      }
    }).catch(function(err) {
      console.warn('PKCE flow background notice:', err);
    });
  });

  document.getElementById('btn-msa-cancel').addEventListener('click', function() {
    msaCompleted = true;
    if (msaPollTimer) clearInterval(msaPollTimer);
    msaPollTimer = null;
    closeModal('modal-msa');
  });

  document.getElementById('btn-add-offline').addEventListener('click', function () {
    closeModal('modal-add-account');
    document.getElementById('offline-username').value = '';
    document.getElementById('offline-error').classList.add('hidden');
    openModal('modal-offline');
    setTimeout(function () { document.getElementById('offline-username').focus(); }, 100);
  });
  document.getElementById('btn-offline-cancel').addEventListener('click', function () { closeModal('modal-offline'); });
  document.getElementById('btn-offline-submit').addEventListener('click', function () {
    var u = document.getElementById('offline-username').value.trim();
    if (!u || u.length < 3 || u.length > 16) { document.getElementById('offline-error').textContent = 'Никнейм: 3-16 символов'; document.getElementById('offline-error').classList.remove('hidden'); return; }
    window.backend.addAccountOffline(u).then(function () { closeModal('modal-offline'); loadAccounts(); }).catch(function (e) { document.getElementById('offline-error').textContent = String(e); document.getElementById('offline-error').classList.remove('hidden'); });
  });
  document.getElementById('offline-username').addEventListener('keydown', function (e) { if (e.key === 'Enter') document.getElementById('btn-offline-submit').click(); });

  function loadAccounts() {
    window.backend.listAccounts().then(function (json) {
      var accs = JSON.parse(json) || [];
      var grid = document.getElementById('accounts-grid');
      grid.innerHTML = '';

      var activeAcc = accs.find(function(a) { return a.is_active; }) || accs[0];

      // Update Sidebar Active Account Widget
      var sName = document.getElementById('active-acc-name');
      var sType = document.getElementById('active-acc-type');
      var sCanvas = document.getElementById('active-acc-head-canvas');

      var offlineTxt = window.i18n ? window.i18n.t('offline_account', 'Офлайн аккаунт') : 'Офлайн аккаунт';
      var noAccTxt = window.i18n ? window.i18n.t('no_account', 'Нет аккаунта') : 'Нет аккаунта';
      var activeTxt = window.i18n ? window.i18n.t('active', 'Активен') : 'Активен';
      var makeActiveTxt = window.i18n ? window.i18n.t('make_active', 'Сделать активным') : 'Сделать активным';
      var noAccountsTxt = window.i18n ? window.i18n.t('no_accounts', 'Нет аккаунтов') : 'Нет аккаунтов';

      if (activeAcc) {
        if (sName) sName.textContent = activeAcc.username;
        if (sType) sType.textContent = activeAcc.account_type === 'microsoft' ? 'Microsoft Account' : offlineTxt;
        drawHeadOnCanvas(sCanvas, activeAcc.skin_png_base64);
      } else {
        if (sName) sName.textContent = '—';
        if (sType) sType.textContent = noAccTxt;
        drawHeadOnCanvas(sCanvas, null);
      }

      if (!accs || accs.length === 0) {
        grid.innerHTML = '<div class="col-span-full text-center py-12"><span class="material-symbols-outlined text-[64px] text-text-secondary">person_off</span><p class="text-headline-md text-on-surface mt-4">' + noAccountsTxt + '</p></div>';
        return;
      }

      accs.forEach(function (a) {
        var card = document.createElement('div');
        card.className = (a.is_active ? 'bg-surface-elevated border border-primary/50' : 'bg-surface-container border border-outline-variant/30') + ' rounded-xl p-5 relative group cursor-pointer transition-all';
        var typeLabel = a.account_type === 'microsoft' ? 'Microsoft Account' : offlineTxt;
        var actions = a.is_active
          ? '<div class="mt-6 flex gap-2"><button class="flex-1 bg-surface-variant py-2 rounded-lg font-label-md text-label-md text-text-secondary cursor-default">' + activeTxt + '</button><button class="btn-delete-acc w-10 h-10 flex items-center justify-center bg-surface-variant hover:bg-status-error/10 hover:text-status-error rounded-lg text-text-secondary transition-colors" title="Удалить аккаунт"><span class="material-symbols-outlined text-[18px]">delete</span></button></div>'
          : '<div class="mt-6 flex gap-2"><button class="btn-activate flex-1 bg-primary/10 text-primary border border-primary/20 hover:bg-primary/20 py-2 rounded-lg font-label-md text-label-md font-bold transition-all">' + makeActiveTxt + '</button><button class="btn-delete-acc w-10 h-10 flex items-center justify-center bg-surface-variant hover:bg-status-error/10 hover:text-status-error rounded-lg text-text-secondary transition-colors" title="Удалить аккаунт"><span class="material-symbols-outlined text-[18px]">delete</span></button></div>';
        card.innerHTML = '<div class="flex items-center gap-4"><canvas class="acc-head-canvas w-12 h-12 rounded-lg bg-surface-variant shrink-0 border border-outline-variant/50" width="48" height="48"></canvas><div class="flex-1 overflow-hidden"><h4 class="font-headline-md text-headline-md text-on-surface truncate">' + esc(a.username) + '</h4><p class="font-label-md text-label-md text-text-secondary">' + typeLabel + '</p></div></div>' + actions;

        var headCanvas = card.querySelector('.acc-head-canvas');
        drawHeadOnCanvas(headCanvas, a.skin_png_base64);

        var actBtn = card.querySelector('.btn-activate');
        if (actBtn) actBtn.addEventListener('click', function (e) { e.stopPropagation(); window.backend.setActiveAccount(a.id).then(loadAccounts); });
        var delBtn = card.querySelector('.btn-delete-acc');
        if (delBtn) delBtn.addEventListener('click', function (e) { e.stopPropagation(); showAlert('Удалить аккаунт?', 'Удалить "' + a.username + '"?', true, 'delete').then(function (ok) { if (ok) window.backend.removeAccount(a.id).then(loadAccounts); }); });
        grid.appendChild(card);
      });
    }).catch(function (e) { console.warn(e); });
  }

  // === Wizard ===
  var wizardVersion = null;
  var wizardLoader = '';
  var wizardLoaderVersion = null;
  var wizardLoaderUnsupported = false;
  var wizardVersionsCache = [];
  var wizardLoaderVersionsCache = [];
  var wizardModpacksLoaded = false;
  var wizardSearchInput = document.getElementById('wizard-search');
  var wizardLoaderSearchInput = document.getElementById('wizard-loader-search');
  var wizardLoaderNames = { fabric: 'Fabric', forge: 'Forge', neoforge: 'NeoForge', quilt: 'Quilt' };

  document.getElementById('btn-add-instance').addEventListener('click', openWizard);
  document.getElementById('btn-create-first').addEventListener('click', openWizard);
  document.getElementById('btn-wizard-cancel').addEventListener('click', function () { closeModal('modal-wizard'); });
  document.getElementById('btn-refresh-versions').addEventListener('click', function () { loadWizardVersions(); });
  document.getElementById('btn-refresh-versions2').addEventListener('click', function () { loadWizardLoaderVersions(); });

  document.querySelectorAll('input[name="loader"]').forEach(function (r) {
    r.addEventListener('change', function () {
      wizardLoader = r.value;
      wizardLoaderVersion = null;
      loadWizardLoaderVersions();
    });
  });

  document.querySelectorAll('[data-filter]').forEach(function (cb) {
    cb.addEventListener('change', function () { loadWizardVersions(); });
  });

  if (wizardSearchInput) wizardSearchInput.addEventListener('input', function () { renderWizardVersions(wizardVersionsCache); });
  if (wizardLoaderSearchInput) wizardLoaderSearchInput.addEventListener('input', function () { renderWizardLoaderVersions(wizardLoaderVersionsCache); });

  function resetWizardToCustomTab() {
    var customBtn = document.querySelector('.wizard-source[data-source="custom"]');
    if (customBtn) {
      document.querySelectorAll('.wizard-source').forEach(function (b) { b.classList.remove('sidebar-item-active'); b.classList.add('text-on-surface-variant'); });
      customBtn.classList.add('sidebar-item-active');
      customBtn.classList.remove('text-on-surface-variant');
    }
  }

  // === Wizard panel switching ===
  var currentWizardSource = 'custom';

  function showWizardPanel(src) {
    currentWizardSource = src;
    // hide all panels
    document.querySelectorAll('.wizard-panel').forEach(function (p) { p.classList.add('hidden'); p.classList.remove('flex'); });
    var panel = document.getElementById('wizard-panel-' + src);
    if (panel) { panel.classList.remove('hidden'); panel.classList.add('flex'); }
    // Show/hide create button — not needed for import/modrinth (they have their own actions)
    var createBtn = document.getElementById('btn-wizard-create');
    if (createBtn) createBtn.classList.toggle('hidden', src === 'import' || src === 'modrinth');
  }

  // Wizard tab sources
  document.querySelectorAll('.wizard-source').forEach(function (btn) {
    btn.addEventListener('click', function () {
      document.querySelectorAll('.wizard-source').forEach(function (b) { b.classList.remove('sidebar-item-active'); b.classList.add('text-on-surface-variant'); });
      btn.classList.add('sidebar-item-active'); btn.classList.remove('text-on-surface-variant');
      var src = btn.dataset.source;
      showWizardPanel(src);
      if (src === 'modrinth' && !wizardModpacksLoaded) {
        searchWizardModpacks('');
      }
    });
  });

  // === Import via native file dialog ===
  function doWizardImport() {
    var statusEl = document.getElementById('wizard-import-status');
    if (statusEl) { statusEl.textContent = 'Открытие диалога...'; statusEl.classList.remove('hidden'); }
    window.backend.openFileDialog().then(function (filePath) {
      if (!filePath) {
        if (statusEl) statusEl.classList.add('hidden');
        return;
      }
      if (statusEl) { statusEl.textContent = 'Импортирование: ' + filePath.split('\\').pop() + '...'; }
      window.backend.importInstance(filePath).then(function (json) {
        var result = JSON.parse(json);
        if (statusEl) statusEl.classList.add('hidden');
        closeModal('modal-wizard');
        loadInstances();
        showAlert('Импорт завершён', '"' + result.name + '" успешно добавлен!', false, 'success');
      }).catch(function (e) {
        if (statusEl) { statusEl.textContent = 'Ошибка: ' + String(e); statusEl.classList.remove('hidden'); }
      });
    }).catch(function (e) {
      if (statusEl) { statusEl.textContent = 'Ошибка: ' + String(e); statusEl.classList.remove('hidden'); }
    });
  }

  var pickFileBtn = document.getElementById('btn-wizard-pick-file');
  if (pickFileBtn) pickFileBtn.addEventListener('click', doWizardImport);
  var dropZone = document.getElementById('wizard-import-drop-zone');
  if (dropZone) dropZone.addEventListener('click', doWizardImport);


  function openWizard() {
    wizardVersion = null; wizardLoader = ''; wizardLoaderVersion = null; wizardLoaderUnsupported = false;
    wizardLoaderVersionsCache = [];
    wizardModpacksLoaded = false;
    currentWizardSource = 'custom';
    document.querySelectorAll('input[name="loader"]').forEach(function (r) { r.checked = r.value === ''; });
    document.getElementById('wizard-name').value = '';
    document.getElementById('wizard-group').value = '';
    if (wizardSearchInput) wizardSearchInput.value = '';
    if (wizardLoaderSearchInput) wizardLoaderSearchInput.value = '';
    resetWizardLoaderPanel();
    // Reset sidebar
    document.querySelectorAll('.wizard-source').forEach(function (b) { b.classList.remove('sidebar-item-active'); b.classList.add('text-on-surface-variant'); });
    var customBtn = document.querySelector('.wizard-source[data-source="custom"]');
    if (customBtn) { customBtn.classList.add('sidebar-item-active'); customBtn.classList.remove('text-on-surface-variant'); }
    showWizardPanel('custom');
    // Reset import status
    var statusEl = document.getElementById('wizard-import-status');
    if (statusEl) { statusEl.textContent = ''; statusEl.classList.add('hidden'); }
    openModal('modal-wizard');
    loadWizardVersions();
  }

  function getActiveFilters() {
    var filters = [];
    document.querySelectorAll('[data-filter]:checked').forEach(function (cb) { filters.push(cb.dataset.filter); });
    return filters;
  }

  function loadWizardVersions() {
    var list = document.getElementById('wizard-version-list');
    list.innerHTML = '<div class="text-center py-8 text-text-secondary">Загрузка...</div>';
    var filters = getActiveFilters();
    var filterParam = filters.length === 0 ? 'none' : filters.length === 4 ? 'all' : filters.join(',');
    window.backend.getMcVersions(filterParam).then(function (json) {
      wizardVersionsCache = JSON.parse(json) || [];
      renderWizardVersions(wizardVersionsCache);
    }).catch(function (e) { list.innerHTML = '<div class="text-center py-8 text-status-error">' + esc(String(e)) + '</div>'; });
  }

  function renderWizardVersions(versions) {
    var list = document.getElementById('wizard-version-list');
    var query = wizardSearchInput ? wizardSearchInput.value.trim().toLowerCase() : '';
    var filtered = query ? versions.filter(function (v) { return v.id.toLowerCase().indexOf(query) !== -1; }) : versions;
    list.innerHTML = '';
    if (!filtered.length) { list.innerHTML = '<div class="text-center py-8 text-text-secondary">Нет версий</div>'; return; }
    filtered.forEach(function (v) {
      var selected = wizardVersion === v.id;
      var typeColor = v.version_type === 'release' ? 'text-status-success' : v.version_type === 'snapshot' ? 'text-status-warning' : 'text-primary';
      var row = document.createElement('div');
      row.className = 'flex version-row text-body-sm items-center cursor-pointer' + (selected ? ' selected' : '');
      row.innerHTML =
        '<div class="px-3 py-1 w-48 border-r border-white/10 flex items-center gap-2 min-w-0">' +
          (selected ? '<span class="material-symbols-outlined text-sm" style="font-variation-settings:\'FILL\' 1">star</span>' : '<span class="w-3.5 shrink-0"></span>') +
          '<span class="truncate">' + esc(v.id) + '</span>' +
        '</div>' +
        '<div class="px-3 py-1 w-32 border-r border-white/10">' + esc(v.release_time ? v.release_time.substring(0, 10) : '') + '</div>' +
        '<div class="px-3 py-1 flex-1 ' + (selected ? '' : typeColor) + '">' + esc(v.version_type) + '</div>';
      row.addEventListener('click', function () {
        wizardVersion = v.id;
        document.getElementById('wizard-name').value = v.id;
        renderWizardVersions(wizardVersionsCache);
        if (wizardLoader) loadWizardLoaderVersions();
      });
      list.appendChild(row);
    });
  }

  function placeholderMarkup(text) {
    return '<div class="flex-1 border border-dashed border-outline-variant rounded flex items-center justify-center bg-surface-container-lowest/30"><div class="bg-surface-container-high px-12 py-6 rounded-lg shadow-lg border border-outline-variant"><p class="text-on-surface-variant font-bold text-headline-md">' + esc(text) + '</p></div></div>';
  }

  function resetWizardLoaderPanel() {
    document.getElementById('wizard-loader-panel').innerHTML = placeholderMarkup('Не выбран загрузчик модов.');
  }

  var wizardLoaderRequestId = 0;

  function loadWizardLoaderVersions() {
    var panel = document.getElementById('wizard-loader-panel');
    wizardLoaderUnsupported = false;
    if (!wizardLoader) { resetWizardLoaderPanel(); return; }
    if (!wizardVersion) { panel.innerHTML = placeholderMarkup('Сначала выберите версию Minecraft.'); return; }
    panel.innerHTML = '<div class="text-center py-8 text-text-secondary">Загрузка...</div>';
    var requestId = ++wizardLoaderRequestId;
    var requestVersion = wizardVersion;
    var requestLoader = wizardLoader;
    window.backend.getModLoaders(requestVersion).then(function (json) {
      // Discard stale responses: user may have switched version/loader while this request was in flight
      if (requestId !== wizardLoaderRequestId || requestVersion !== wizardVersion || requestLoader !== wizardLoader) return;

      var loaders = JSON.parse(json) || [];
      var name = wizardLoaderNames[wizardLoader] || wizardLoader;
      var match = loaders.find(function (l) { return l.name.toLowerCase() === name.toLowerCase(); });

      // If no loaders exist for selected version, display exact requirement message
      if (!loaders.length || !match || !match.versions || !match.versions.length) {
        wizardLoaderUnsupported = true;
        wizardLoaderVersion = null;
        panel.innerHTML = placeholderMarkup('Нет доступных загрузчиков');
        return;
      }

      wizardLoaderVersionsCache = (match && match.versions) || [];
      renderWizardLoaderVersions(wizardLoaderVersionsCache);
    }).catch(function (e) {
      if (requestId !== wizardLoaderRequestId || requestVersion !== wizardVersion || requestLoader !== wizardLoader) return;
      panel.innerHTML = '<div class="text-center py-8 text-status-error">' + esc(String(e)) + '</div>';
    });
  }

  function renderWizardLoaderVersions(versions) {
    var panel = document.getElementById('wizard-loader-panel');
    if (!wizardLoader) { resetWizardLoaderPanel(); return; }
    if (!versions.length) {
      wizardLoaderUnsupported = true;
      wizardLoaderVersion = null;
      panel.innerHTML = placeholderMarkup('Нет доступных загрузчиков');
      return;
    }
    wizardLoaderUnsupported = false;
    var query = wizardLoaderSearchInput ? wizardLoaderSearchInput.value.trim().toLowerCase() : '';
    var filtered = query ? versions.filter(function (v) { return v.version.toLowerCase().indexOf(query) !== -1; }) : versions;
    if (!filtered.length) { panel.innerHTML = '<div class="text-center py-8 text-text-secondary">Нет доступных загрузчиков</div>'; return; }
    var container = document.createElement('div');
    container.className = 'version-table-container flex-1 overflow-y-auto rounded-sm';
    filtered.forEach(function (v) {
      var selected = wizardLoaderVersion === v.version;
      var row = document.createElement('div');
      row.className = 'flex version-row text-body-sm items-center justify-between px-3 py-1.5 cursor-pointer border-b border-white/10' + (selected ? ' selected' : '');
      row.innerHTML = '<span class="truncate">' + esc(v.version) + '</span>' + (selected ? '<span class="material-symbols-outlined text-sm">check</span>' : '');
      row.addEventListener('click', function () { wizardLoaderVersion = v.version; renderWizardLoaderVersions(wizardLoaderVersionsCache); });
      container.appendChild(row);
    });
    panel.innerHTML = '';
    panel.appendChild(container);
  }

  document.getElementById('btn-wizard-create').addEventListener('click', function () {
    var name = document.getElementById('wizard-name').value.trim();
    var group = document.getElementById('wizard-group').value.trim() || null;
    if (!wizardVersion) { showAlert('Ошибка', 'Выберите версию', false, 'error'); return; }
    if (wizardLoader && !wizardLoaderUnsupported && !wizardLoaderVersion) { showAlert('Ошибка', 'Выберите версию загрузчика модов', false, 'error'); return; }
    if (!name) name = wizardVersion;
    window.backend.createInstance({ name: name, mc_version: wizardVersion, loader: wizardLoader || null, loader_version: wizardLoaderVersion, group: group })
      .then(function () { closeModal('modal-wizard'); loadInstances(); })
      .catch(function (e) { showAlert('Ошибка', String(e), false, 'error'); });
  });

  // === Main Screen Mods Browser ===
  var modSearchInput = document.getElementById('mod-search-input');
  var modLoaderFilter = document.getElementById('mod-loader-filter');
  var modResults = document.getElementById('mod-results');
  var modPage = 0;
  var modLoading = false;

  function initModBrowser() {
    if (modSearchInput) modSearchInput.onkeydown = function (e) { if (e.key === 'Enter') { modPage = 0; searchMods(); } };
    if (modLoaderFilter) modLoaderFilter.onchange = function () { modPage = 0; searchMods(); };
    var scrollContainer = document.getElementById('mod-scroll-container');
    if (scrollContainer) scrollContainer.onscroll = function (e) { if (e.target.scrollTop + e.target.clientHeight >= e.target.scrollHeight - 200) loadMoreMods(); };
    if (modResults && !modResults.dataset.loaded) { modResults.dataset.loaded = '1'; modPage = 0; loadPopularMods(); }
  }

  function loadPopularMods() {
    modResults.innerHTML = '<div class="text-center py-12 text-text-secondary"><div class="animate-spin w-8 h-8 border-2 border-primary border-t-transparent rounded-full mx-auto mb-4"></div><p>Загрузка...</p></div>';
    window.backend.searchMods('', '', '', 0).then(function (json) { renderMods(JSON.parse(json)); }).catch(function (e) { modResults.innerHTML = '<div class="text-center py-12 text-status-error">' + esc(String(e)) + '</div>'; });
  }

  function searchMods() {
    var q = modSearchInput.value.trim(); if (!q) return;
    modResults.innerHTML = '<div class="text-center py-12 text-text-secondary"><div class="animate-spin w-8 h-8 border-2 border-primary border-t-transparent rounded-full mx-auto mb-4"></div></div>';
    modPage = 0;
    window.backend.searchMods(q, '', modLoaderFilter.value, 0).then(function (json) { modResults.innerHTML = ''; renderModsAppend(JSON.parse(json)); }).catch(function (e) { modResults.innerHTML = '<div class="text-center py-12 text-status-error">' + esc(String(e)) + '</div>'; });
  }

  function loadMoreMods() {
    if (modLoading) return; modLoading = true; modPage++;
    window.backend.searchMods(modSearchInput.value.trim(), '', modLoaderFilter.value, modPage).then(function (json) { renderModsAppend(JSON.parse(json)); modLoading = false; }).catch(function () { modLoading = false; });
  }

  function renderMods(mods) { if (!mods || !mods.length) { modResults.innerHTML = '<div class="text-center py-12 text-text-secondary">Ничего не найдено</div>'; return; } modResults.innerHTML = ''; renderModsAppend(mods); }

  function renderModsAppend(mods) {
    if (!mods || !mods.length) return;
    mods.forEach(function (m) {
      var card = document.createElement('div');
      card.className = 'flex items-start gap-4 p-4 bg-surface-card rounded-xl border border-outline-variant hover:border-outline transition-colors cursor-pointer';
      var icon = m.icon_url ? '<img src="' + esc(m.icon_url) + '" class="w-12 h-12 rounded-lg object-cover bg-surface-container-high"/>' : '<div class="w-12 h-12 rounded-lg bg-surface-container-high flex items-center justify-center"><span class="material-symbols-outlined text-text-secondary">extension</span></div>';
      card.innerHTML = icon + '<div class="flex-1 min-w-0"><h3 class="font-headline-md text-headline-md font-bold text-on-surface truncate">' + esc(m.title) + '</h3><p class="text-body-sm text-text-secondary mt-1 line-clamp-2">' + esc(m.description) + '</p><div class="flex items-center gap-4 mt-2"><span class="font-label-sm text-label-sm text-text-secondary">by ' + esc(m.author) + '</span><span class="font-label-sm text-label-sm text-text-secondary flex items-center gap-1"><span class="material-symbols-outlined text-[14px]">download</span>' + m.downloads + '</span></div></div>' +
        '<button class="btn-install px-4 py-2 bg-primary-container text-on-primary-container rounded-lg font-label-md text-label-md font-bold hover:brightness-110 transition-all shrink-0" data-slug="' + esc(m.id) + '" data-title="' + esc(m.title) + '">Установить</button>';
      modResults.appendChild(card);
    });
    document.querySelectorAll('#mod-results .btn-install').forEach(function (btn) {
      if (btn.dataset.bound) return; btn.dataset.bound = '1';
      btn.addEventListener('click', function (e) {
        e.stopPropagation();
        var targetSelect = document.getElementById('mod-target-instance');
        var targetId = (targetSelect && targetSelect.value) ? targetSelect.value : selectedInstanceId;
        if (!targetId) { showAlert('Выберите сборку', 'Пожалуйста, выберите целевую сборку в верхней панели или выберите сборку на главном экране', false, 'info'); return; }
        installModToInstance(btn.dataset.slug, btn.dataset.title, targetId);
      });
    });
  }

  // === Wizard Modrinth Modpacks ===
  var wizardModpackSearch = document.getElementById('wizard-modpack-search');
  var wizardModpackLoaderFilter = document.getElementById('wizard-modpack-loader-filter');
  var wizardModpackList = document.getElementById('wizard-modpack-list');

  window.searchWizardModpacks = function(query) {
    if (!wizardModpackList) return;
    var loader = wizardModpackLoaderFilter ? wizardModpackLoaderFilter.value : '';
    wizardModpackList.innerHTML = '<div class="text-center py-12 text-text-secondary"><div class="animate-spin w-8 h-8 border-2 border-primary border-t-transparent rounded-full mx-auto mb-4"></div><p>Загрузка...</p></div>';
    
    window.backend.searchModpacks(query || '', loader || '', 0).then(function(json) {
      wizardModpacksLoaded = true;
      var modpacks = JSON.parse(json) || [];
      wizardModpackList.innerHTML = '';
      if (!modpacks.length) {
        wizardModpackList.innerHTML = '<div class="text-center py-12 text-text-secondary">Ничего не найдено</div>';
        return;
      }
      modpacks.forEach(function(m) {
        var card = document.createElement('div');
        card.className = 'flex items-center gap-3 p-3 bg-surface-card rounded-xl border border-outline-variant hover:border-primary/50 transition-colors cursor-pointer';
        var icon = m.icon_url ? '<img src="' + esc(m.icon_url) + '" class="w-12 h-12 rounded-lg object-cover bg-surface-container-high shrink-0"/>' : '<div class="w-12 h-12 rounded-lg bg-surface-container-high flex items-center justify-center shrink-0"><span class="material-symbols-outlined text-text-secondary text-sm">extension</span></div>';
        card.innerHTML = icon + '<div class="flex-1 min-w-0"><h4 class="font-body-md font-bold text-on-surface truncate">' + esc(m.title) + '</h4><p class="text-label-sm text-text-secondary truncate">' + esc(m.description) + '</p></div>';
        
        card.addEventListener('click', function() {
          showWizardModpackDetail(m);
        });
        
        wizardModpackList.appendChild(card);
      });
    }).catch(function(e) {
      wizardModpackList.innerHTML = '<div class="text-center py-12 text-status-error">' + esc(String(e)) + '</div>';
    });
  };

  if (wizardModpackSearch) {
    wizardModpackSearch.addEventListener('keydown', function(e) {
      if (e.key === 'Enter') window.searchWizardModpacks(wizardModpackSearch.value.trim());
    });
  }
  if (wizardModpackLoaderFilter) {
    wizardModpackLoaderFilter.addEventListener('change', function() {
      window.searchWizardModpacks(wizardModpackSearch ? wizardModpackSearch.value.trim() : '');
    });
  }

  function showWizardModpackDetail(m) {
    var detail = document.getElementById('wizard-modpack-detail');
    if (!detail) return;
    
    var icon = m.icon_url ? '<img src="' + esc(m.icon_url) + '" class="w-16 h-16 rounded-xl object-cover bg-surface-container-high mx-auto mb-3"/>' : '<div class="w-16 h-16 rounded-xl bg-surface-container-high flex items-center justify-center mx-auto mb-3"><span class="material-symbols-outlined text-[32px] text-text-secondary">extension</span></div>';
    
    detail.innerHTML = '<div class="text-center">' + icon + '<h3 class="font-headline-md font-bold text-on-surface">' + esc(m.title) + '</h3><p class="text-label-sm text-text-secondary mb-4">by ' + esc(m.author) + '</p></div>' +
      '<div class="bg-surface-container border border-outline-variant rounded-lg p-3 text-body-sm text-on-surface mb-4">' + esc(m.description) + '</div>' + 
      '<div class="text-center mb-2"><div class="animate-spin w-6 h-6 border-2 border-primary border-t-transparent rounded-full mx-auto"></div><p class="text-label-sm text-text-secondary mt-2">Загрузка версий...</p></div>';
      
    window.backend.getModpackVersions(m.id).then(function(json) {
      var versions = JSON.parse(json) || [];
      if (!versions.length) {
        detail.innerHTML = '<div class="text-center">' + icon + '<h3 class="font-headline-md font-bold text-on-surface">' + esc(m.title) + '</h3><p class="text-status-error mt-4">Нет доступных версий</p></div>';
        return;
      }
      
      var options = versions.map(function(v, idx) {
        var mcList = v.mc_versions && v.mc_versions.length ? ' [' + v.mc_versions.slice(0, 2).join(', ') + ']' : '';
        return '<option value="' + idx + '">' + esc(v.name || v.version) + mcList + '</option>';
      }).join('');
      
      detail.innerHTML = '<div class="text-center">' + icon + '<h3 class="font-headline-md font-bold text-on-surface">' + esc(m.title) + '</h3><p class="text-label-sm text-text-secondary mb-4">by ' + esc(m.author) + '</p></div>' +
        '<div class="bg-surface-container border border-outline-variant rounded-lg p-3 text-body-sm text-on-surface mb-4 overflow-y-auto max-h-24">' + esc(m.description) + '</div>' + 
        '<label class="text-label-sm text-text-secondary block mb-1">Выберите версию сборки:</label>' +
        '<select id="wizard-modpack-version-select" class="w-full bg-surface-dim border border-outline-variant rounded-lg px-3 py-2 text-body-sm text-on-surface focus:outline-none mb-4">' + options + '</select>' +
        '<button id="btn-wizard-modpack-install" class="w-full py-2.5 bg-primary-container text-white font-bold rounded-lg hover:brightness-110 active:scale-95 transition-all">Скачать и установить</button>';
        
      document.getElementById('btn-wizard-modpack-install').addEventListener('click', function() {
        var sel = document.getElementById('wizard-modpack-version-select');
        var idx = parseInt(sel.value, 10);
        var v = versions[idx];
        if (v && v.download_url) {
          installWizardModpack(v.download_url, v.filename, m.title, m.icon_url);
        } else {
          showAlert('Ошибка', 'Нет ссылки для скачивания', false, 'error');
        }
      });
    }).catch(function(e) {
      detail.innerHTML = '<div class="text-center">' + icon + '<h3 class="font-headline-md font-bold text-on-surface">' + esc(m.title) + '</h3><p class="text-status-error mt-4">' + esc(String(e)) + '</p></div>';
    });
  }

  function installWizardModpack(url, filename, title, iconUrl) {
    var detail = document.getElementById('wizard-modpack-detail');
    var statusEl = document.getElementById('wizard-import-status');
    if (!statusEl && detail) {
      statusEl = document.createElement('div');
      statusEl.id = 'wizard-import-status';
      statusEl.className = 'text-body-sm text-primary mt-4 font-bold text-center';
      detail.appendChild(statusEl);
    }
    if (statusEl) {
      statusEl.textContent = 'Скачивание ' + title + '...';
      statusEl.classList.remove('hidden', 'text-status-error');
      statusEl.classList.add('text-primary');
    }
    
    var btn = document.getElementById('btn-wizard-modpack-install');
    if (btn) btn.disabled = true;

    window.backend.importModpackFromUrl(url, filename, iconUrl).then(function(json) {
      var result = JSON.parse(json);
      if (statusEl) statusEl.classList.add('hidden');
      closeModal('modal-wizard');
      loadInstances();
      showAlert('Успех', '"' + result.name + '" успешно установлена!', false, 'success');
    }).catch(function(e) {
      if (statusEl) {
        statusEl.textContent = 'Ошибка: ' + String(e);
        statusEl.classList.remove('text-primary');
        statusEl.classList.add('text-status-error');
      }
      if (btn) btn.disabled = false;
    });
  }

  // === Import ZIP / MRPACK ===
  function doImport(cb) {
    var input = document.createElement('input');
    input.type = 'file';
    input.accept = '.zip,.mrpack';
    input.style.display = 'none';
    document.body.appendChild(input);

    var handled = false;

    function finish(cancelled) {
      if (handled) return;
      handled = true;
      if (document.body.contains(input)) {
        document.body.removeChild(input);
      }
      if (typeof cb === 'function') cb(cancelled);
    }

    input.addEventListener('change', function () {
      if (input.files && input.files.length > 0) {
        var path = input.files[0].path || input.files[0].name;
        window.backend.importInstance(path).then(function (r) {
          showAlert('Импорт', JSON.parse(r).message, false, 'success');
          loadInstances();
          closeModal('modal-wizard');
          finish(false);
        }).catch(function (e) {
          showAlert('Ошибка', String(e), false, 'error');
          finish(false);
        });
      } else {
        finish(true);
      }
    });

    if ('oncancel' in input) {
      input.addEventListener('cancel', function () {
        finish(true);
      });
    }

    window.addEventListener('focus', function onFocus() {
      window.removeEventListener('focus', onFocus);
      setTimeout(function () {
        if (!handled && (!input.files || input.files.length === 0)) {
          finish(true);
        }
      }, 350);
    }, { once: true });

    input.click();
  }
  var btnImportFirst = document.getElementById('btn-import-first');
  if (btnImportFirst) btnImportFirst.addEventListener('click', function () { doImport(); });

  // === Init ===
  loadInstances();
  loadAccounts();
  if (window.i18n) window.i18n.applyLanguage();
  initLauncherSettings();
  function esc(s) { if (!s) return ''; var d = document.createElement('div'); d.textContent = s; return d.innerHTML; }
})();
