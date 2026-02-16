import type { Card, Transaction, Category, User } from '$lib/types';
import {
  createCard,
  createCategory,
  createTransaction,
  createSession,
  clearSession,
  deleteCard,
  deleteCategory,
  deleteTransaction,
  fetchCards,
  fetchCategories,
  fetchTransactionsPage,
  fetchUserSummary,
} from '$lib/api';

const TRANSACTIONS_PAGE_SIZE = 50;

function createAppStore() {
  let user = $state<User | null>(null);
  let isBootstrapping = $state(true);
  let cards = $state<Card[]>([]);
  let transactions = $state<Transaction[]>([]);
  let categories = $state<Category[]>([]);
  let transactionsOffset = $state(0);
  let transactionsHasMore = $state(true);
  let transactionsLoading = $state(false);

  const loadAll = async () => {
    transactionsLoading = true;
    try {
      const [cardsData, categoriesData, transactionsData] = await Promise.all([
        fetchCards(),
        fetchCategories(),
        fetchTransactionsPage({ limit: TRANSACTIONS_PAGE_SIZE, offset: 0 }),
      ]);
      cards = cardsData;
      categories = categoriesData;
      transactions = transactionsData;
      transactionsOffset = transactionsData.length;
      transactionsHasMore = transactionsData.length === TRANSACTIONS_PAGE_SIZE;
    } finally {
      transactionsLoading = false;
    }
  };

  return {
    // User
    get user() {
      return user;
    },
    get isAuthenticated() {
      return user !== null;
    },
    get isBootstrapping() {
      return isBootstrapping;
    },
    async loginWithToken(token: string) {
      user = await createSession(token);
      await loadAll();
    },
    async bootstrap() {
      isBootstrapping = true;
      try {
        user = await fetchUserSummary();
        await loadAll();
      } catch (err) {
        user = null;
        cards = [];
        transactions = [];
        categories = [];
        transactionsOffset = 0;
        transactionsHasMore = true;
        transactionsLoading = false;
        throw err;
      } finally {
        isBootstrapping = false;
      }
    },
    logout() {
      clearSession().catch((err) => {
        console.error('Failed to clear session', err);
      });
      user = null;
      cards = [];
      transactions = [];
      categories = [];
      transactionsOffset = 0;
      transactionsHasMore = true;
      transactionsLoading = false;
    },

    // Cards
    get cards() {
      return cards;
    },
    async addCard(card: Omit<Card, 'id'>) {
      const newCard = await createCard(card);
      cards.push(newCard);
      return newCard;
    },
    async deleteCard(id: string) {
      await deleteCard(id);
      cards = cards.filter((c) => c.id !== id);
      const removedCount = transactions.filter((t) => t.cardId === id).length;
      transactions = transactions.filter((t) => t.cardId !== id);
      transactionsOffset = Math.max(0, transactionsOffset - removedCount);
    },
    getCard(id: string) {
      return cards.find((c) => c.id === id);
    },

    // Transactions
    get transactions() {
      return transactions;
    },
    get transactionsHasMore() {
      return transactionsHasMore;
    },
    get transactionsLoading() {
      return transactionsLoading;
    },
    async loadMoreTransactions(limit = TRANSACTIONS_PAGE_SIZE) {
      if (transactionsLoading || !transactionsHasMore) {
        return;
      }

      transactionsLoading = true;
      try {
        const nextPage = await fetchTransactionsPage({
          limit,
          offset: transactionsOffset,
        });
        const existingIds = new Set(transactions.map((t) => t.id));
        const uniqueTransactions = nextPage.filter((t) => !existingIds.has(t.id));
        transactions = [...transactions, ...uniqueTransactions];
        transactionsOffset += nextPage.length;
        transactionsHasMore = nextPage.length === limit;
      } finally {
        transactionsLoading = false;
      }
    },
    async addTransaction(transaction: Omit<Transaction, 'id'>) {
      const newTransaction = await createTransaction(transaction);
      transactions = [...transactions, newTransaction];
      transactionsOffset += 1;

      // Update card balance
      const card = cards.find((c) => c.id === newTransaction.cardId);
      if (card) {
        card.currentBalance += newTransaction.amount;
      }
      return newTransaction;
    },
    async deleteTransaction(id: string) {
      const transaction = transactions.find((t) => t.id === id);
      await deleteTransaction(id);
      if (transaction) {
        // Reverse the balance change
        const card = cards.find((c) => c.id === transaction.cardId);
        if (card) {
          card.currentBalance -= transaction.amount;
        }
      }
      transactions = transactions.filter((t) => t.id !== id);
      if (transaction) {
        transactionsOffset = Math.max(0, transactionsOffset - 1);
      }
    },
    getTransactionsByCard(cardId: string) {
      return transactions.filter((t) => t.cardId === cardId);
    },
    getTransactionsByCategory(categoryId: string) {
      return transactions.filter((t) => t.categoryId === categoryId);
    },

    // Categories
    get categories() {
      return categories;
    },
    async addCategory(category: Omit<Category, 'id'>) {
      const newCategory = await createCategory(category);
      categories = [...categories, newCategory];
      return newCategory;
    },
    async deleteCategory(id: string) {
      await deleteCategory(id);
      categories = categories.filter((c) => c.id !== id);
    },
    getCategory(id: string) {
      return categories.find((c) => c.id === id);
    },
  };
}

export const appStore = createAppStore();
