import type { BarChartProps, PieChartProps } from 'layerchart';

export interface OneAxisChartValues<T = Record<string, unknown>> extends Omit<
	PieChartProps<T>,
	'data' | 'key' | 'value'
> {
	data: T[] | undefined;
	key: keyof T;
	value: keyof T;
}

export interface TwoAxisChartValues<T = Record<string, unknown>> extends Omit<
	BarChartProps<T>,
	'data' | 'x' | 'y'
> {
	data: T[] | null | undefined;
	x: keyof T;
	y: keyof T;
}
