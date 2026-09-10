type Key = number | string;

type Node<K extends Key, T> = {
	key: K;
	value: T;
	left: Node<K, T> | null;
	right: Node<K, T> | null;
};

class BinaryTree<K extends Key, T> {
	#tree: Node<K, T> | null = null;
	#count: number = 0;

	#createNode<K extends Key, T>(key: K, value: T): Node<K, T> {
		return {
			key,
			value,
			left: null,
			right: null
		};
	}

	#insertNumber(currentNode: Node<number, T>, newNode: Node<number, T>) {
		if (newNode.key < currentNode.key) {
			if (currentNode.left === null) {
				currentNode.left = newNode;
				return;
			}
			this.#insertNumber(currentNode.left, newNode);
		} else if (newNode.key > currentNode.key) {
			if (currentNode.right === null) {
				currentNode.right = newNode;
				return;
			}
			this.#insertNumber(currentNode.right, newNode);
		} else {
			console.error(`Duplicate key: ${currentNode}, ${newNode}`);
		}
	}

	#insertString(currentNode: Node<string, T>, newNode: Node<string, T>) {
		const check = currentNode.key.localeCompare(newNode.key);

		if (check > 0) {
			if (currentNode.left === null) {
				currentNode.left = newNode;
				return;
			}
			this.#insertString(currentNode.left, newNode);
		} else if (check < 0) {
			if (currentNode.right === null) {
				currentNode.right = newNode;
				return;
			}
			this.#insertString(currentNode.right, newNode);
		} else {
			console.error(`Duplicate key: ${currentNode}, ${newNode}`);
		}
	}

	insert(key: K, value?: T) {
		const newNode = this.#createNode(key, value ?? key) as Node<K, T>;

		if (this.#tree === null) {
			this.#tree = newNode;
			return;
		}

		if (typeof key === 'number') {
			this.#insertNumber(this.#tree as Node<number, T>, newNode as Node<number, T>);
			this.#count += 1;
			return;
		}

		if (typeof key === 'string') {
			this.#insertString(this.#tree as Node<string, T>, newNode as Node<string, T>);
			this.#count += 1;
			return;
		}

		console.error('Incorrect key type: should be string or number');
	}

	get count() {
		return this.#count;
	}

	#buildArray(node: Node<K, T>, arr: T[]): void {
		if (node.left !== null) {
			this.#buildArray(node.left, arr);
		}

		arr.push(node.value);

		if (node.right !== null) {
			this.#buildArray(node.right, arr);
		}
	}

	toArray(): T[] {
		if (this.#tree === null) {
			return [];
		}

		const arr: T[] = [];
		this.#buildArray(this.#tree, arr);
		return arr;
	}
}

export default BinaryTree;
