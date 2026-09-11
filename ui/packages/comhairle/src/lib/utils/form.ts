type OnSubmit = SubmitEvent & { currentTarget: EventTarget & HTMLFormElement };

export const handleSubmit = (callback: (formData: FormData) => void) => (e: OnSubmit) => {
	const formData = new FormData(e.currentTarget);
	callback(formData);
};
