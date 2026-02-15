import type { Card, Category, Transaction, User } from '$lib/types';

const API_BASE = '/api';

type ApiCard = {
  id: string;
  userEmail: string;
  name: string;
  cardType: 'credit' | 'debit';
  creditLimit?: number | null;
  currentBalance: number;
};

type ApiCategory = {
  id: string;
  userEmail?: string | null;
  name: string;
  color?: string | null;
};

type ApiTransaction = {
  id: string;
  userEmail: string;
  cardId: string;
  categoryId: string;
  amount: number;
  description: string;
  transactionType: 'expense' | 'income' | 'payment';
  date: string;
};

type ApiUserClaims = {
  email: string;
  name: string;
  picture: string;
};

const centsToDollars = (value: number | null | undefined) =>
  typeof value === 'number' ? value / 100 : undefined;
const dollarsToCents = (value: number | null | undefined) =>
  typeof value === 'number' ? Math.round(value * 100) : undefined;

const toCard = (card: ApiCard): Card => ({
  id: card.id,
  name: card.name,
  type: card.cardType,
  creditLimit: centsToDollars(card.creditLimit) ?? undefined,
  currentBalance: (centsToDollars(card.currentBalance) ?? 0),
});

const toCategory = (category: ApiCategory): Category => ({
  id: category.id,
  name: category.name,
  color: category.color ?? undefined,
});

const toTransaction = (tx: ApiTransaction): Transaction => ({
  id: tx.id,
  cardId: tx.cardId,
  categoryId: tx.categoryId,
  amount: (centsToDollars(tx.amount) ?? 0),
  description: tx.description,
  type: tx.transactionType,
  date: new Date(tx.date),
});

const toUser = (claims: ApiUserClaims): User => ({
  id: claims.email,
  email: claims.email,
  name: claims.name,
  avatarUrl: claims.picture,
});

async function apiFetch<T>(path: string, options: RequestInit = {}): Promise<T> {
  const { headers, ...rest } = options;
  const response = await fetch(`${API_BASE}${path}`, {
    ...rest,
    headers: {
      'Content-Type': 'application/json',
      ...headers,
    },
    credentials: 'include',
  });

  if (response.status === 204) {
    return null as T;
  }

  if (!response.ok) {
    const message = await response.text();
    throw new Error(message || `Request failed: ${response.status}`);
  }

  return response.json() as Promise<T>;
}

export async function fetchUserSummary(): Promise<User> {
  const claims = await apiFetch<ApiUserClaims>('/user/summary');
  return toUser(claims);
}

export async function fetchCards(): Promise<Card[]> {
  const cards = await apiFetch<ApiCard[]>('/card');
  return cards.map(toCard);
}

export async function createCard(card: Omit<Card, 'id'>): Promise<Card> {
  const payload = {
    name: card.name,
    cardType: card.type,
    creditLimit: dollarsToCents(card.creditLimit),
  };
  const created = await apiFetch<ApiCard>('/card', {
    method: 'POST',
    body: JSON.stringify(payload),
  });
  return toCard(created);
}

export async function deleteCard(id: string): Promise<void> {
  await apiFetch<void>(`/card/${id}`, { method: 'DELETE' });
}

export async function fetchCategories(): Promise<Category[]> {
  const categories = await apiFetch<ApiCategory[]>('/category');
  return categories.map(toCategory);
}

export async function createCategory(category: Omit<Category, 'id'>): Promise<Category> {
  const payload = {
    name: category.name,
    color: category.color ?? null,
  };
  const created = await apiFetch<ApiCategory>('/category', {
    method: 'POST',
    body: JSON.stringify(payload),
  });
  return toCategory(created);
}

export async function deleteCategory(id: string): Promise<void> {
  await apiFetch<void>(`/category/${id}`, { method: 'DELETE' });
}

type FetchTransactionsParams = {
  limit?: number;
  offset?: number;
};

export async function fetchTransactionsPage(
  params: FetchTransactionsParams = {},
): Promise<Transaction[]> {
  const search = new URLSearchParams();
  if (params.limit) {
    search.set('limit', `${params.limit}`);
  }
  if (params.offset) {
    search.set('offset', `${params.offset}`);
  }
  const query = search.toString();
  const path = query ? `/transaction?${query}` : '/transaction';
  const transactions = await apiFetch<ApiTransaction[]>(path);
  return transactions.map(toTransaction);
}

export async function createTransaction(
  transaction: Omit<Transaction, 'id'>,
): Promise<Transaction> {
  const payload = {
    cardId: transaction.cardId,
    categoryId: transaction.categoryId,
    amount: dollarsToCents(transaction.amount) ?? 0,
    description: transaction.description,
    transactionType: transaction.type,
    date: transaction.date.toISOString(),
  };
  const created = await apiFetch<ApiTransaction>('/transaction', {
    method: 'POST',
    body: JSON.stringify(payload),
  });
  return toTransaction(created);
}

export async function deleteTransaction(id: string): Promise<void> {
  await apiFetch<void>(`/transaction/${id}`, { method: 'DELETE' });
}

export async function createSession(token: string): Promise<User> {
  const claims = await apiFetch<ApiUserClaims>('/user/session', {
    method: 'POST',
    body: JSON.stringify({ token }),
  });
  return toUser(claims);
}

export async function clearSession(): Promise<void> {
  await apiFetch<void>('/user/session', { method: 'DELETE' });
}
