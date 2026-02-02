export type CardType = 'credit' | 'debit';

export interface Card {
	id: string;
	name: string;
	type: CardType;
	creditLimit?: number; // Only for credit cards
	currentBalance: number;
}

export interface Category {
	id: string;
	name: string;
	icon?: string;
	color?: string;
}

export interface Transaction {
	id: string;
	cardId: string;
	categoryId: string;
	amount: number;
	description: string;
	date: Date;
	type: 'expense' | 'income' | 'payment'; // payment = paying off credit card
}

export interface User {
	id: string;
	email: string;
	name: string;
	avatarUrl?: string;
}
