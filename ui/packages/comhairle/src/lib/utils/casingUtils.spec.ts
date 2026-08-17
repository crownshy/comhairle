import { describe, expect, it } from 'vitest';
import { camelToSnakeCase, recursiveCaseChangeKeys, snakeToCamel } from './casingUtils';

describe('casingUtils', () => {
	describe('recursiveCaseChangeKeys', () => {
		it('Should convert all keys from to snake_case to camelCase', () => {
			const payload = {
				first_snake: 'test',
				second_snake: 'test',
				nested_arr: [{ nested_snake: 'test' }, { nested_snake: 'test' }],
				nested_obj: {
					nested_snake: 'test'
				}
			};

			const result = recursiveCaseChangeKeys(payload, snakeToCamel);

			expect(result.firstSnake).toBe('test');
			expect(result.secondSnake).toBe('test');
			expect(result.nestedArr[0].nestedSnake).toBe('test');
			expect(result.nestedArr[1].nestedSnake).toBe('test');
			expect(result.nestedObj.nestedSnake).toBe('test');
		});

		it('Should convert all keys from to camelCase to snake_case', () => {
			const payload = {
				firstCamel: 'test',
				secondCamel: 'test',
				nestedArr: [{ nestedCamel: 'test' }, { nestedCamel: 'test' }],
				nestedObj: {
					nestedCamel: 'test'
				}
			};

			const result = recursiveCaseChangeKeys(payload, camelToSnakeCase);

			expect(result.first_camel).toBe('test');
			expect(result.second_camel).toBe('test');
			expect(result.nested_arr[0].nested_camel).toBe('test');
			expect(result.nested_arr[1].nested_camel).toBe('test');
			expect(result.nested_obj.nested_camel).toBe('test');
		});
	});
});
