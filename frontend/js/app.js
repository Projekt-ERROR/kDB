// ─────────────────────────────────────────────
//  kDB App Logic
// ─────────────────────────────────────────────
import * as api from './api.js';

// ── State ─────────────────────────────────────
const state = {
  kitchen: [],
  ingredients: [],
  activeTab: 'kitchen',
  editMode: 'add',
  filterCat: 'all',
  searchQuery: '',
  editSearchQuery: '',
  removeSearchQuery: '',
  catalogueSearchQuery: '',
  selectedIngredient: null,
  loading: false,
  toast: null,
};

// ── Helpers ───────────────────────────────────

function today() {
  return new Date().toISOString().split('T')[0];
}

function daysUntil(dateStr) {
  if (!dateStr) return null;
  const now = new Date(); now.setHours(0,0,0,0);
  const d   = new Date(dateStr); d.setHours(0,0,0,0);
  return Math.round((d - now) / 86400000);
}

function expiryStatus(entry) {
  const days = daysUntil(entry.expires_on);
  if (days === null) return 'none';
  if (days < 0)     return 'expired';
  if (days <= 1)    return 'urgent';
  if (days <= 3)    return 'warn';
  return 'good';
}

function expiryLabel(entry) {
  const days = daysUntil(entry.expires_on);
  if (days === null) return 'no expiry set';
  if (days < 0)     return `expired ${Math.abs(days)}d ago`;
  if (days === 0)   return 'expires today';
  if (days === 1)   return 'expires tomorrow';
  return `expires in ${days} days`;
}

// Two-letter abbreviation instead of emoji
function categoryAbbr(cat) {
  if (!cat) return '??';
  return cat.trim().slice(0, 2).toUpperCase();
}

function ingredientFor(entry) {
  return state.ingredients.find(i => i.id === entry.ingredient_id) || null;
}

function kitchenView() {
  return state.kitchen.map(entry => ({
    ...entry,
    ingredient: ingredientFor(entry),
    status: expiryStatus(entry),
    expiryLabel: expiryLabel(entry),
  }));
}

// ── Toast ─────────────────────────────────────
function showToast(msg, type = 'info') {
  const el = document.getElementById('toast');
  el.textContent = msg;
  el.className = `toast toast-${type} show`;
  clearTimeout(state.toast);
  state.toast = setTimeout(() => el.classList.remove('show'), 3000);
}

// ── Loading bar ───────────────────────────────
function setLoading(on) {
  state.loading = on;
  document.getElementById('loading-bar').style.width = on ? '70%' : '100%';
  if (!on) setTimeout(() => {
    document.getElementById('loading-bar').style.width = '0';
  }, 300);
}

// ── Confirm modal (no browser confirm()) ──────
let pendingConfirm = null;

function showConfirm(message, onConfirm) {
  pendingConfirm = onConfirm;
  document.getElementById('confirm-message').textContent = message;
  document.getElementById('confirm-modal').classList.add('open');
}

function closeConfirm() {
  document.getElementById('confirm-modal').classList.remove('open');
  pendingConfirm = null;
}

function acceptConfirm() {
  const fn = pendingConfirm;
  closeConfirm();
  if (fn) fn();
}

// ── Edit ingredient modal ─────────────────────
function openEditIngredient(id) {
  const ing = state.ingredients.find(i => i.id === id);
  if (!ing) return;
  document.getElementById('edit-ing-id').value       = ing.id;
  document.getElementById('edit-ing-name').value     = ing.name;
  document.getElementById('edit-ing-category').value = ing.category || '';
  document.getElementById('edit-ing-unit').value     = ing.default_unit || '';
  document.getElementById('edit-ing-shelf').value    = ing.shelf_life_days || '';
  document.getElementById('edit-ing-storage').value  = ing.storage || '';
  document.getElementById('edit-ingredient-modal').classList.add('open');
  document.getElementById('edit-ingredient-modal-overlay').classList.add('open');
  document.getElementById('edit-ing-name').focus();
}

function closeEditIngredient() {
  document.getElementById('edit-ingredient-modal').classList.remove('open');
  document.getElementById('edit-ingredient-modal-overlay').classList.remove('open');
}

async function submitEditIngredient(e) {
  e.preventDefault();
  const id       = document.getElementById('edit-ing-id').value;
  const name     = document.getElementById('edit-ing-name').value.trim();
  const category = document.getElementById('edit-ing-category').value.trim() || null;
  const unit     = document.getElementById('edit-ing-unit').value.trim() || null;
  const shelf    = parseInt(document.getElementById('edit-ing-shelf').value) || null;
  const storage  = document.getElementById('edit-ing-storage').value.trim() || null;

  if (!name) return showToast('name is required', 'error');

  try {
    const updated = await api.ingredients.update(id, {
      name, category, default_unit: unit, shelf_life_days: shelf, storage,
    });
    const idx = state.ingredients.findIndex(i => i.id === id);
    if (idx !== -1) state.ingredients[idx] = updated;
    state.ingredients.sort((a, b) => a.name.localeCompare(b.name));
    closeEditIngredient();
    showToast(`updated ${updated.name}`, 'success');
    renderKitchen();
    renderRemoveList();
    renderAddSearch();
    renderCatalogue();
    if (state.activeTab === 'stats') renderStats();
  } catch (err) {
    showToast(`error: ${err.message}`, 'error');
  }
}

// ── Render: Kitchen Tab ───────────────────────
function renderKitchen() {
  const view = kitchenView();

  let filtered = view;
  if (state.filterCat !== 'all') {
    filtered = filtered.filter(v =>
      (v.ingredient?.category || '').toLowerCase().includes(state.filterCat)
    );
  }
  if (state.searchQuery) {
    const q = state.searchQuery.toLowerCase();
    filtered = filtered.filter(v =>
      (v.ingredient?.name || '').toLowerCase().includes(q)
    );
  }

  const order = { expired: 0, urgent: 1, warn: 2, good: 3, none: 4 };
  filtered.sort((a, b) => (order[a.status] ?? 5) - (order[b.status] ?? 5));

  const expiringSoon = view.filter(v => v.status === 'urgent' || v.status === 'warn').length;
  const expired      = view.filter(v => v.status === 'expired').length;
  const totalCount   = view.length;

  document.getElementById('badge-expiring').textContent =
    expiringSoon > 0 ? `${expiringSoon} expiring` : expired > 0 ? `${expired} expired` : 'all good';
  document.getElementById('badge-expiring').className =
    'header-badge' + (expiringSoon > 0 || expired > 0 ? ' urgent' : '');
  document.getElementById('badge-count').textContent = `${totalCount} items`;
  document.getElementById('nav-dot').style.display =
    (expiringSoon > 0 || expired > 0) ? 'block' : 'none';

  const banner = document.getElementById('expiry-banner');
  if (expiringSoon > 0 || expired > 0) {
    const parts = [];
    if (expired > 0)      parts.push(`<strong>${expired}</strong> expired`);
    if (expiringSoon > 0) parts.push(`<strong>${expiringSoon}</strong> expiring soon`);
    document.getElementById('expiry-banner-text').innerHTML = parts.join(', ');
    banner.style.display = 'flex';
  } else {
    banner.style.display = 'none';
  }

  document.getElementById('section-count').textContent = `${filtered.length} items`;
  const list = document.getElementById('card-list');

  if (filtered.length === 0) {
    const empty = state.searchQuery || state.filterCat !== 'all';
    list.innerHTML = `
      <div class="empty-state">
        <div class="empty-title">${empty ? 'no results' : 'kitchen is empty'}</div>
        <div class="empty-sub">${empty ? 'try a different filter' : 'add items via the edit tab'}</div>
      </div>`;
    return;
  }

  list.innerHTML = filtered.map((v, i) => {
    const name    = v.ingredient?.name || 'Unknown';
    const storage = v.ingredient?.storage || '—';
    const cat     = v.ingredient?.category || null;
    const safe    = name.replace(/\\/g, '\\\\').replace(/'/g, "\\'");
    return `
      <div class="inv-card status-${v.status}" style="animation-delay:${i * 40}ms" data-id="${v.id}">
        <div class="inv-card-icon">${categoryAbbr(cat)}</div>
        <div class="inv-card-body">
          <div class="inv-card-name">${name}${v.opened ? '<span class="opened-badge">open</span>' : ''}</div>
          <div class="inv-card-meta">${v.quantity} ${v.unit} · ${storage}</div>
          <div class="inv-card-expiry ${v.status}">${v.expiryLabel}</div>
        </div>
        <div class="inv-card-actions">
          <div class="qty-btn" onclick="App.toggleOpened('${v.id}', ${v.opened})"
            title="${v.opened ? 'mark closed' : 'mark opened'}">${v.opened ? 'cls' : 'opn'}</div>
          <div class="qty-btn remove" onclick="App.deleteEntry('${v.id}', '${safe}')">&#x2715;</div>
        </div>
      </div>`;
  }).join('');
}

// ── Render: Edit Tab search results ──────────
function renderAddSearch() {
  const q = state.editSearchQuery.toLowerCase().trim();
  const resultsEl = document.getElementById('add-search-results');

  if (!q) { resultsEl.style.display = 'none'; return; }

  const matches = state.ingredients.filter(i => i.name.toLowerCase().includes(q)).slice(0, 6);

  if (matches.length === 0) {
    resultsEl.innerHTML = `<div class="search-result-item no-match">no matches — use "define new" below</div>`;
    resultsEl.style.display = 'block';
    return;
  }

  resultsEl.innerHTML = matches.map(i => {
    const meta = [i.category, i.storage, i.shelf_life_days ? `${i.shelf_life_days}d shelf life` : null]
      .filter(Boolean).join(' · ');
    return `
      <div class="search-result-item">
        <span class="result-abbr" onclick="App.selectIngredient('${i.id}')">${categoryAbbr(i.category)}</span>
        <div class="result-info" onclick="App.selectIngredient('${i.id}')">
          <div class="result-name">${i.name}</div>
          <div class="result-cat">${meta}</div>
        </div>
        <span class="result-unit" onclick="App.selectIngredient('${i.id}')">${i.default_unit || '—'}</span>
        <span class="result-edit-btn" onclick="App.openEditIngredient('${i.id}')">edit</span>
      </div>`;
  }).join('');
  resultsEl.style.display = 'block';
}

// ── Render: Remove mode ───────────────────────
function renderRemoveList() {
  const q = state.removeSearchQuery.toLowerCase().trim();
  const view = kitchenView();
  const filtered = q
    ? view.filter(v => (v.ingredient?.name || '').toLowerCase().includes(q))
    : view;

  const list = document.getElementById('remove-card-list');
  if (filtered.length === 0) {
    list.innerHTML = `<div class="empty-state"><div class="empty-title">nothing to remove</div></div>`;
    return;
  }

  list.innerHTML = filtered.map((v, i) => {
    const name = v.ingredient?.name || 'Unknown';
    const cat  = v.ingredient?.category || null;
    const safe = name.replace(/\\/g, '\\\\').replace(/'/g, "\\'");
    return `
      <div class="remove-card" style="animation-delay:${i * 35}ms">
        <div class="remove-card-icon">${categoryAbbr(cat)}</div>
        <div class="remove-card-body">
          <div class="remove-card-name">${name}</div>
          <div class="remove-card-meta">${v.quantity} ${v.unit} · ${v.expiryLabel}</div>
        </div>
        <div class="qty-control">
          <div class="qty-btn remove" onclick="App.deleteEntry('${v.id}', '${safe}')">&#x2715; remove</div>
        </div>
      </div>`;
  }).join('');
}

// ── Render: Stats Tab ─────────────────────────
function renderStats() {
  const view = kitchenView();
  const total    = view.length;
  const expired  = view.filter(v => v.status === 'expired').length;
  const expiring = view.filter(v => v.status === 'urgent' || v.status === 'warn').length;

  document.getElementById('stat-total').textContent     = total;
  document.getElementById('stat-expired').textContent   = expired;
  document.getElementById('stat-expiring').textContent  = expiring;
  document.getElementById('stat-catalogue').textContent = state.ingredients.length;

  const catCounts = {};
  view.forEach(v => {
    const cat = v.ingredient?.category || 'other';
    catCounts[cat] = (catCounts[cat] || 0) + 1;
  });

  const catList = document.getElementById('stat-categories');
  if (Object.keys(catCounts).length === 0) {
    catList.innerHTML = `<div class="empty-sub" style="padding:6px 0;font-size:11px;color:var(--text-dim)">no data yet</div>`;
  } else {
    const sorted = Object.entries(catCounts).sort((a, b) => b[1] - a[1]);
    const max = sorted[0][1];
    catList.innerHTML = sorted.map(([cat, count]) => `
      <div class="bar-row">
        <span class="bar-label">${cat}</span>
        <div class="bar-track"><div class="bar-fill" style="width:${Math.round(count/max*100)}%"></div></div>
        <span class="bar-val">${count}</span>
      </div>`).join('');
  }

  const expiryList = document.getElementById('stat-expiry-breakdown');
  const urgentItems = view.filter(v => ['expired','urgent','warn'].includes(v.status));
  if (urgentItems.length === 0) {
    expiryList.innerHTML = `<div class="empty-sub" style="padding:6px 0;font-size:11px;color:var(--text-dim)">all good</div>`;
  } else {
    expiryList.innerHTML = urgentItems.slice(0, 5).map(v => `
      <div class="bar-row">
        <span class="bar-label">${v.ingredient?.name || '?'}</span>
        <div class="bar-track" style="flex:1"><div class="bar-fill red" style="width:100%"></div></div>
        <span class="bar-val" style="color:var(--${v.status === 'good' ? 'green' : 'red'})">${v.expiryLabel.replace('expires ','')}</span>
      </div>`).join('');
  }
}

// ── Render: Catalogue Tab ────────────────────
function renderCatalogue() {
  const q = state.catalogueSearchQuery.toLowerCase().trim();
  const list = document.getElementById('catalogue-list');

  let items = state.ingredients;
  if (q) items = items.filter(i => i.name.toLowerCase().includes(q) || (i.category || '').toLowerCase().includes(q));

  if (items.length === 0) {
    list.innerHTML = `
      <div class="empty-state">
        <div class="empty-title">${q ? 'no results' : 'no ingredients defined'}</div>
        <div class="empty-sub">${q ? 'try a different search' : 'define ingredients in the edit tab'}</div>
      </div>`;
    return;
  }

  list.innerHTML = items.map((ing, i) => {
    const meta = [ing.category, ing.storage, ing.shelf_life_days ? `${ing.shelf_life_days}d shelf life` : null, ing.default_unit ? `default: ${ing.default_unit}` : null]
      .filter(Boolean).join(' · ');
    const inKitchen = state.kitchen.filter(e => e.ingredient_id === ing.id).length;
    return `
      <div class="cat-card" style="animation-delay:${i * 30}ms">
        <div class="cat-card-abbr">${categoryAbbr(ing.category)}</div>
        <div class="cat-card-body">
          <div class="cat-card-name">${ing.name}${inKitchen > 0 ? `<span class="cat-in-kitchen">${inKitchen} in kitchen</span>` : ''}</div>
          <div class="cat-card-meta">${meta || '—'}</div>
        </div>
        <div class="cat-card-actions">
          <div class="qty-btn" onclick="App.openEditIngredient('${ing.id}')">edit</div>
          <div class="qty-btn remove" onclick="App.deleteIngredient('${ing.id}', '${ing.name.replace(/\\/g, '\\\\').replace(/'/g, "\\'")}', ${inKitchen})">&#x2715;</div>
        </div>
      </div>`;
  }).join('');
}

async function deleteIngredient(id, name, inKitchen) {
  const msg = inKitchen > 0
    ? `delete "${name}"? this will also remove ${inKitchen} kitchen item${inKitchen > 1 ? 's' : ''}.`
    : `delete "${name}" from catalogue?`;
  showConfirm(msg, async () => {
    try {
      await api.ingredients.delete(id);
      state.ingredients = state.ingredients.filter(i => i.id !== id);
      // kitchen entries with this ingredient_id are cascade-deleted by the DB
      state.kitchen = state.kitchen.filter(e => e.ingredient_id !== id);
      renderCatalogue();
      renderKitchen();
      renderRemoveList();
      renderStats();
      showToast(`deleted ${name}`, 'success');
    } catch (e) {
      showToast(`error: ${e.message}`, 'error');
    }
  });
}



async function loadAll() {
  setLoading(true);
  try {
    const [k, i] = await Promise.all([api.kitchen.list(), api.ingredients.list()]);
    state.kitchen     = k;
    state.ingredients = i;
    renderKitchen();
    renderRemoveList();
    renderStats();
    renderCatalogue();
  } catch (e) {
    showToast(`failed to load: ${e.message}`, 'error');
  } finally {
    setLoading(false);
  }
}

async function deleteEntry(id, name) {
  showConfirm(`remove "${name}" from kitchen?`, async () => {
    try {
      await api.kitchen.delete(id);
      state.kitchen = state.kitchen.filter(e => e.id !== id);
      renderKitchen();
      renderRemoveList();
      renderStats();
      showToast(`removed ${name}`, 'success');
    } catch (e) {
      showToast(`error: ${e.message}`, 'error');
    }
  });
}

async function clearKitchen() {
  if (state.kitchen.length === 0) { showToast('kitchen is already empty', 'info'); return; }
  showConfirm(`clear all ${state.kitchen.length} items from kitchen?`, async () => {
    try {
      await Promise.all(state.kitchen.map(e => api.kitchen.delete(e.id)));
      state.kitchen = [];
      renderKitchen();
      renderRemoveList();
      renderStats();
      showToast('kitchen cleared', 'success');
    } catch (e) {
      showToast(`error: ${e.message}`, 'error');
    }
  });
}

async function toggleOpened(id, currentlyOpened) {
  try {
    const updated = await api.kitchen.update(id, { opened: !currentlyOpened });
    const idx = state.kitchen.findIndex(e => e.id === id);
    if (idx !== -1) state.kitchen[idx] = updated;
    renderKitchen();
  } catch (e) {
    showToast(`error: ${e.message}`, 'error');
  }
}

function selectIngredient(id) {
  const ing = state.ingredients.find(i => i.id === id);
  if (!ing) return;
  state.selectedIngredient = ing;

  document.getElementById('add-ingredient-display').textContent = ing.name;
  document.getElementById('add-ingredient-id').value = ing.id;
  document.getElementById('add-unit').value = ing.default_unit || '';
  document.getElementById('add-purchased').value = today();

  if (ing.shelf_life_days) {
    const exp = new Date();
    exp.setDate(exp.getDate() + ing.shelf_life_days);
    document.getElementById('add-expires').value = exp.toISOString().split('T')[0];
  } else {
    document.getElementById('add-expires').value = '';
  }

  document.getElementById('add-search-results').style.display = 'none';
  document.getElementById('add-search-input').value = '';
  document.getElementById('add-search-input').placeholder = 'ingredient selected';
  document.getElementById('selected-ingredient-pill').style.display = 'flex';
  document.getElementById('add-form-fields').style.display = 'block';
  document.getElementById('add-quantity').focus();
}

async function submitAdd(e) {
  e.preventDefault();
  const ingId    = document.getElementById('add-ingredient-id').value;
  const quantity = parseFloat(document.getElementById('add-quantity').value);
  const unit     = document.getElementById('add-unit').value.trim();
  const purchased = document.getElementById('add-purchased').value;
  const expires  = document.getElementById('add-expires').value || null;
  const opened   = document.getElementById('add-opened-toggle').classList.contains('on');

  if (!ingId)                           return showToast('select an ingredient first', 'error');
  if (isNaN(quantity) || quantity <= 0) return showToast('enter a valid quantity', 'error');
  if (!unit)                            return showToast('unit is required', 'error');

  try {
    const entry = await api.kitchen.create({
      ingredient_id: ingId, quantity, unit,
      purchased_on: purchased || undefined,
      expires_on: expires || undefined,
      opened,
    });
    state.kitchen.push(entry);
    renderKitchen(); renderRemoveList(); renderStats();
    showToast(`added ${state.selectedIngredient?.name}`, 'success');
    resetAddForm();
  } catch (e) {
    showToast(`error: ${e.message}`, 'error');
  }
}

function resetAddForm() {
  state.selectedIngredient = null;
  document.getElementById('add-search-input').value = '';
  document.getElementById('add-search-input').placeholder = 'search ingredients catalogue...';
  document.getElementById('add-ingredient-id').value = '';
  document.getElementById('add-ingredient-display').textContent = '';
  document.getElementById('selected-ingredient-pill').style.display = 'none';
  document.getElementById('add-quantity').value = '';
  document.getElementById('add-unit').value = '';
  document.getElementById('add-purchased').value = today();
  document.getElementById('add-expires').value = '';
  document.getElementById('add-opened-toggle').classList.remove('on');
  document.getElementById('add-form-fields').style.display = 'none';
  document.getElementById('add-search-results').style.display = 'none';
}

async function submitNewIngredient(e) {
  e.preventDefault();
  const name     = document.getElementById('new-ing-name').value.trim();
  const category = document.getElementById('new-ing-category').value.trim() || null;
  const unit     = document.getElementById('new-ing-unit').value.trim() || null;
  const shelf    = parseInt(document.getElementById('new-ing-shelf').value) || null;
  const storage  = document.getElementById('new-ing-storage').value.trim() || null;

  if (!name) return showToast('name is required', 'error');

  try {
    const ing = await api.ingredients.create({ name, category, default_unit: unit, shelf_life_days: shelf, storage });
    state.ingredients.push(ing);
    state.ingredients.sort((a,b) => a.name.localeCompare(b.name));
    showToast(`created ${ing.name}`, 'success');
    closeNewIngredientForm();
    selectIngredient(ing.id);
    renderCatalogue();
  } catch (e) {
    showToast(`error: ${e.message}`, 'error');
  }
}

function closeNewIngredientForm() {
  document.getElementById('new-ingredient-form').style.display = 'none';
  ['new-ing-name','new-ing-category','new-ing-unit','new-ing-shelf','new-ing-storage']
    .forEach(id => { document.getElementById(id).value = ''; });
}

// ── Tab switching ─────────────────────────────
const TAB_ORDER = ['kitchen', 'edit', 'catalogue', 'stats'];

function switchTab(tab) {
  if (tab === state.activeTab) return;
  const prev = TAB_ORDER.indexOf(state.activeTab);
  const next = TAB_ORDER.indexOf(tab);

  document.querySelectorAll('.tab-content').forEach(el =>
    el.classList.remove('active', 'anim-right', 'anim-left', 'anim-up')
  );
  document.querySelectorAll('.nav-btn').forEach(el => el.classList.remove('active'));

  const el = document.getElementById(`tab-${tab}`);
  el.classList.add('active');
  el.classList.add(tab === 'edit' ? 'anim-up' : next > prev ? 'anim-right' : 'anim-left');
  document.getElementById(`nav-${tab}`).classList.add('active');
  state.activeTab = tab;

  if (tab === 'stats') renderStats();
  if (tab === 'catalogue') renderCatalogue();
  if (tab === 'edit' && state.editMode === 'remove') renderRemoveList();
}

function setEditMode(mode) {
  state.editMode = mode;
  document.getElementById('mode-add').classList.toggle('active-add', mode === 'add');
  document.getElementById('mode-add').classList.remove('active-remove');
  document.getElementById('mode-remove').classList.toggle('active-remove', mode === 'remove');
  document.getElementById('mode-remove').classList.remove('active-add');
  document.getElementById('add-mode').classList.toggle('visible', mode === 'add');
  document.getElementById('remove-mode').classList.toggle('visible', mode === 'remove');
  if (mode === 'remove') renderRemoveList();
}

function toggleSettings() {
  document.getElementById('settings-panel').classList.toggle('open');
  document.getElementById('settings-overlay').classList.toggle('open');
  document.getElementById('settings-btn').classList.toggle('active');
}

function setFilter(cat) {
  state.filterCat = cat;
  document.querySelectorAll('.filter-pill').forEach(p =>
    p.classList.toggle('active', p.dataset.cat === cat)
  );
  renderKitchen();
}

function onKitchenSearch(val)   { state.searchQuery = val; renderKitchen(); }
function onEditSearch(val)      { state.editSearchQuery = val; renderAddSearch(); }
function onRemoveSearch(val)    { state.removeSearchQuery = val; renderRemoveList(); }
function onCatalogueSearch(val) { state.catalogueSearchQuery = val; renderCatalogue(); }

// ── Init ──────────────────────────────────────
async function init() {
  document.getElementById('add-purchased').value = today();

  document.getElementById('kitchen-search').addEventListener('input', e => onKitchenSearch(e.target.value));
  document.getElementById('add-search-input').addEventListener('input', e => onEditSearch(e.target.value));
  document.getElementById('remove-search-input').addEventListener('input', e => onRemoveSearch(e.target.value));
  document.getElementById('catalogue-search').addEventListener('input', e => onCatalogueSearch(e.target.value));
  document.getElementById('add-form').addEventListener('submit', submitAdd);
  document.getElementById('new-ingredient-form').addEventListener('submit', submitNewIngredient);
  document.getElementById('edit-ingredient-form').addEventListener('submit', submitEditIngredient);

  document.getElementById('add-opened-toggle').addEventListener('click', function() {
    this.classList.toggle('on');
  });
  document.getElementById('show-new-ingredient-btn').addEventListener('click', () => {
    const form = document.getElementById('new-ingredient-form');
    form.style.display = form.style.display === 'none' ? 'block' : 'none';
    if (form.style.display === 'block') document.getElementById('new-ing-name').focus();
  });

  document.getElementById('cancel-new-ingredient').addEventListener('click', closeNewIngredientForm);
  document.getElementById('close-edit-ingredient').addEventListener('click', closeEditIngredient);
  document.getElementById('edit-ingredient-modal-overlay').addEventListener('click', e => {
    if (e.target === document.getElementById('edit-ingredient-modal-overlay')) closeEditIngredient();
  });

  document.getElementById('confirm-cancel').addEventListener('click', closeConfirm);
  document.getElementById('confirm-ok').addEventListener('click', acceptConfirm);

  document.getElementById('clear-kitchen-btn').addEventListener('click', () => {
    toggleSettings();
    setTimeout(clearKitchen, 200); // let settings panel close first
  });

  document.querySelectorAll('.s-toggle').forEach(t =>
    t.addEventListener('click', () => t.classList.toggle('on'))
  );

  await loadAll();
}

// ── Public API ────────────────────────────────
window.App = {
  switchTab, setEditMode, toggleSettings, setFilter,
  deleteEntry, deleteIngredient, toggleOpened, selectIngredient, openEditIngredient,
};

document.addEventListener('DOMContentLoaded', init);
