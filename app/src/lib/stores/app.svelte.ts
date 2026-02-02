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
	fetchTransactions,
	fetchUserSummary,
} from '$lib/api';

function createAppStore() {
	let user = $state<User | null>(null);
	let isBootstrapping = $state(true);
	let cards = $state<Card[]>([]);
	let transactions = $state<Transaction[]>([]);
	let categories = $state<Category[]>([]);

	const loadAll = async () => {
		const [cardsData, categoriesData, transactionsData] = await Promise.all([
			fetchCards(),
			fetchCategories(),
			fetchTransactions(),
		]);
		cards = cardsData;
		categories = categoriesData;
		transactions = transactionsData;
	};

	return {
		// User
		get user() { return user; },
		get isAuthenticated() { return user !== null; },
		get isBootstrapping() { return isBootstrapping; },
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
		},

		// Cards
		get cards() { return cards; },
		async addCard(card: Omit<Card, 'id'>) {
			const newCard = await createCard(card);
			cards = [...cards, newCard];
			return newCard;
		},
		async deleteCard(id: string) {
			await deleteCard(id);
			cards = cards.filter(c => c.id !== id);
			transactions = transactions.filter(t => t.cardId !== id);
		},
		getCard(id: string) {
			return cards.find(c => c.id === id);
		},

		// Transactions
		get transactions() { return transactions; },
		async addTransaction(transaction: Omit<Transaction, 'id'>) {
			const newTransaction = await createTransaction(transaction);
			transactions = [...transactions, newTransaction];

			// Update card balance
			const card = cards.find(c => c.id === newTransaction.cardId);
			if (card) {
				card.currentBalance += newTransaction.amount;
			}
			return newTransaction;
		},
		async deleteTransaction(id: string) {
			const transaction = transactions.find(t => t.id === id);
			await deleteTransaction(id);
			if (transaction) {
				// Reverse the balance change
				const card = cards.find(c => c.id === transaction.cardId);
				if (card) {
					card.currentBalance -= transaction.amount;
				}
			}
			transactions = transactions.filter(t => t.id !== id);
		},
		getTransactionsByCard(cardId: string) {
			return transactions.filter(t => t.cardId === cardId);
		},
		getTransactionsByCategory(categoryId: string) {
			return transactions.filter(t => t.categoryId === categoryId);
		},

		// Categories
		get categories() { return categories; },
		async addCategory(category: Omit<Category, 'id'>) {
			const newCategory = await createCategory(category);
			categories = [...categories, newCategory];
			return newCategory;
		},
		async deleteCategory(id: string) {
			await deleteCategory(id);
			categories = categories.filter(c => c.id !== id);
		},
		getCategory(id: string) {
			return categories.find(c => c.id === id);
		},
	};
}

export const appStore = createAppStore();
