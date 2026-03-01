// ─────────────────────────────────────────────
//  kDB API client
//  All calls go through here. Change BASE_URL to
//  your Tailscale IP once deployed.
// ─────────────────────────────────────────────

// In production, API calls are relative (proxied via nginx at /kdb/api/).
// For local dev, set backend URL in the settings panel to http://localhost:5000
const BASE_URL = window.KDB_API_URL || '';

async function request(method, path, body = null) {
  const opts = {
    method,
    headers: { 'Content-Type': 'application/json' },
  };
  if (body !== null) opts.body = JSON.stringify(body);

  const res = await fetch(`${BASE_URL}${path}`, opts);

  if (res.status === 204) return null;

  const data = await res.json();

  if (!res.ok) {
    const msg = data?.error || `HTTP ${res.status}`;
    throw new Error(msg);
  }

  return data;
}

// ------------ HEALTH ------------
export const health = {
  check: () => request('GET', '/health'),
};

// ------------ INGREDIENTS ------------
export const ingredients = {
  list:   ()       => request('GET',    '/api/ingredients'),
  get:    (id)     => request('GET',    `/api/ingredients/${id}`),
  create: (body)   => request('POST',   '/api/ingredients', body),
  update: (id, body) => request('PUT',  `/api/ingredients/${id}`, body),
  delete: (id)     => request('DELETE', `/api/ingredients/${id}`),
};

// ------------ KITCHEN ------------
export const kitchen = {
  list:   ()       => request('GET',    '/api/kitchen'),
  get:    (id)     => request('GET',    `/api/kitchen/${id}`),
  create: (body)   => request('POST',   '/api/kitchen', body),
  update: (id, body) => request('PUT',  `/api/kitchen/${id}`, body),
  delete: (id)     => request('DELETE', `/api/kitchen/${id}`),
};

// ------------ RECIPE ------------
export const reciipe = {
  list:   ()       => request('GET',     '/api/recipes'),
  get:    (id)     => request('GET',     `/api/recipes/${id}`),
  create: (body)   => request('POST',    '/api/recipes', body),
  update: (id, body) => request('PUT',   `/api/recipes/${id}`, body),
  delete: (id)     => request('DELETE',  `/api/recipes/${id}`),
  update_ingredients: (id, body) => request('PUT', `/api/recipes/${id}/ingredients`, body),
  update_steps: (id, body) => request('PUT', `/api/recipes/${id}/steps`, body),
  update_tags: (id, body) => request('PUT', `/api/recipes/${id}/tags`, body),
}

// ------------ MEALPLAN ------------
export const mealplan = {
  list:   ()       => request('GET',    '/api/meal-plan'),
  get:    (id)     => request('GET',    `/api/meal-plan/${id}`),
  create: (body)   => request('POST',   '/api/meal-plan', body),
  update: (id, body) => request('PUT',  `/api/meal-plan/${id}`, body),
  delete: (id)     => request('DELETE', `/api/meal-plan/${id}`),
}
