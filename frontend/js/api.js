// ─────────────────────────────────────────────
//  kDB API client
//  All calls go through here. Change BASE_URL to
//  your Tailscale IP once deployed.
// ─────────────────────────────────────────────

const BASE_URL = window.KDB_API_URL || 'http://localhost:5000';

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

// ── Health ──────────────────────────────────
export const health = {
  check: () => request('GET', '/health'),
};

// ── Ingredients (catalogue) ─────────────────
export const ingredients = {
  list:   ()       => request('GET',    '/api/ingredients'),
  get:    (id)     => request('GET',    `/api/ingredients/${id}`),
  create: (body)   => request('POST',   '/api/ingredients', body),
  update: (id, body) => request('PUT',  `/api/ingredients/${id}`, body),
  delete: (id)     => request('DELETE', `/api/ingredients/${id}`),
};

// ── Kitchen (inventory) ─────────────────────
export const kitchen = {
  list:   ()       => request('GET',    '/api/kitchen'),
  get:    (id)     => request('GET',    `/api/kitchen/${id}`),
  create: (body)   => request('POST',   '/api/kitchen', body),
  update: (id, body) => request('PUT',  `/api/kitchen/${id}`, body),
  delete: (id)     => request('DELETE', `/api/kitchen/${id}`),
};
