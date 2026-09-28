import { describe, it, expect } from 'vitest';
import BinaryTree from './BinaryTree';

describe('Binary Tree', () => {
	it('should return a sorted numerical array', () => {
		const binaryTree = new BinaryTree();
		binaryTree.insert(5);
		binaryTree.insert(2);
		binaryTree.insert(10);
		binaryTree.insert(7);
		binaryTree.insert(1);

		expect(binaryTree.toArray()).toStrictEqual([1, 2, 5, 7, 10]);
	});

	it('should return a sorted string array', () => {
		const binaryTree = new BinaryTree();
		binaryTree.insert('orange', { key: 'orange', value: 1 });
		binaryTree.insert('mango', { key: 'mango', value: 2 });
		binaryTree.insert('banana', { key: 'banana', value: 4 });
		binaryTree.insert('apple', { key: 'apple', value: 3 });
		binaryTree.insert('kiwi', { key: 'kiwi', value: 5 });

		expect(binaryTree.toArray()).toStrictEqual([
			{ key: 'apple', value: 3 },
			{ key: 'banana', value: 4 },
			{ key: 'kiwi', value: 5 },
			{ key: 'mango', value: 2 },
			{ key: 'orange', value: 1 }
		]);
	});
});
