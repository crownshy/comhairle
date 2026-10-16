import createContext from '$lib/createContext';

interface AddStepDialogContext {
	open: () => void;
}

export const [getAddStepDialogContext, setAddStepDialogContext] =
	createContext<AddStepDialogContext>();
