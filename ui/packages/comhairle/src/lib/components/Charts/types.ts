import type { BarChartProps, PieChartProps } from 'layerchart';

export interface OneAxisChartValues<T = Record<string, unknown>> extends Omit<
	PieChartProps<T>,
	'data' | 'key' | 'value'
> {
	data: T[] | undefined;
	key: keyof T;
	value: keyof T;
}

type Series<T> = {
	key: T;
	label: string;
	colour: 'primary' | 'secondary' | 'tertiary' | 'quarternary' | 'quinary';
};

export type TwoAxisChartValues<T = Record<string, unknown>> = Omit<
	BarChartProps<T>,
	'data' | 'x' | 'y'
> & {
	config:
		| {
				type: 'normal';
				data: T[] | undefined;
				x: keyof T;
				y: keyof T;
				series?: undefined;
		  }
		| {
				type: 'xSeries';
				data: T[] | undefined;
				x: keyof T;
				y?: undefined;
				series: Series<keyof T>[];
		  }
		| {
				type: 'ySeries';
				data: T[] | undefined;
				x?: undefined;
				y: keyof T;
				series: Series<keyof T>[];
		  };
};
