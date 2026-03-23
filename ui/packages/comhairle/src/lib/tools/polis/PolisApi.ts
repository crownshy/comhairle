export type PolisStatement = {
	txt: string;
	tid: number;
	is_meta: boolean;
	is_seen: boolean;
	lang: string | undefined;
	remaining: number;
	total: number;
};

export type PolisApiState = {
	currentStatement: PolisStatement | undefined;
	loading: boolean;
	error: string | undefined;
	remaining: number;
	total: number;
	ready: boolean;
	pid: number | undefined;
};

export default class PolisApi {
	private _currentStatement: PolisStatement | undefined = undefined;
	private _loading = false;
	private _error: string | undefined = undefined;
	private _remaining = 0;
	private _total = 0;
	private _ready = false;
	private _pid: number | undefined = undefined;

	private polisId: string;
	private userId: string;
	private lang: string;
	private baseUrl: string;
	private onChange: (state: PolisApiState) => void;

	constructor(
		userId: string,
		polisId: string,
		onChange: (state: PolisApiState) => void,
		lang: string = 'en',
		baseUrl: string = 'https://polis.comhairle.scot',
		initialPid?: number
	) {
		this.polisId = polisId;
		this.userId = userId;
		this.lang = lang;
		this.baseUrl = baseUrl.startsWith('https://') ? baseUrl : `https://${baseUrl}`;
		this.onChange = onChange;
		if (initialPid !== undefined) this._pid = initialPid;
		queueMicrotask(() => {
			this.tryToGetPidForXid();
			this.fetchNextStatement();
		});
	}

	private notify() {
		this.onChange({
			currentStatement: this._currentStatement,
			loading: this._loading,
			error: this._error,
			remaining: this._remaining,
			total: this._total,
			ready: this._ready,
			pid: this._pid
		});
	}

	private tryToGetPidForXid() {
		fetch(
			`${this.baseUrl}/api/v3/participationInit?conversation_id=${this.polisId}&xid=${this.userId}`,
			{ credentials: 'omit' }
		)
			.then((r) => {
				if (!r.ok) throw new Error(`participationInit failed: ${r.status}`);
				return r.json();
			})
			.then((data) => {
				if (data.ptpt?.pid) {
					this._pid = data.ptpt.pid;
					this.notify();
				}
			})
			.catch((err) => {
				console.warn('[PolisApi] participationInit failed:', err);
			});
	}

	fetchNextStatement() {
		this._loading = true;
		this._error = undefined;
		this.notify();

		const pidParam = this._pid !== undefined ? `&not_voted_by_pid=${this._pid}` : '';
		const url = `${this.baseUrl}/api/v3/nextComment?conversation_id=${this.polisId}${pidParam}`;

		fetch(url, { credentials: 'omit' })
			.then((s) => {
				if (!s.ok) throw new Error(`nextComment failed: ${s.status}`);
				return s.json();
			})
			.then((comment) => {
				if (typeof comment.currentPid === 'number') {
					this._pid = comment.currentPid;
				}
				if (comment.txt) {
					this._currentStatement = comment;
					this._remaining = comment.remaining;
					this._total = comment.total;
					this._ready = true;
				} else {
					this._currentStatement = undefined;
					this._remaining = 0;
					this._ready = true;
				}
			})
			.catch((err) => {
				console.error('[PolisApi] Failed to fetch next statement:', err);
				this._error = err.message;
			})
			.finally(() => {
				this._loading = false;
				this.notify();
			});
	}

	submitStatement(statement: string) {
		this._loading = true;
		this._error = undefined;
		this.notify();

		const authType = this._pid ? { pid: this._pid } : { xid: this.userId };

		fetch(`${this.baseUrl}/api/v3/comments`, {
			method: 'POST',
			credentials: 'omit',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				conversation_id: this.polisId,
				txt: statement,
				vote: -1,
				is_seed: false,
				...authType
			})
		})
			.then((r) => {
				if (!r.ok) throw new Error(`submitStatement failed: ${r.status}`);
				return r.json();
			})
			.then((data) => {
				if (data.currentPid) {
					this._pid = data.currentPid;
				}
			})
			.catch((err) => {
				console.error('[PolisApi] Error submitting statement:', err);
				this._error = err.message;
			})
			.finally(() => {
				this._loading = false;
				this.notify();
			});
	}

	submitVote(vote: 'agree' | 'disagree' | 'pass') {
		if (!this._currentStatement) {
			console.error('[PolisApi] No current statement to vote on');
			return;
		}

		const votedTid = this._currentStatement.tid;
		this._loading = true;
		this._error = undefined;
		this.notify();

		const voteValue = { agree: -1, disagree: 1, pass: 0 }[vote];
		const authType = this._pid ? { pid: this._pid } : { xid: this.userId };

		fetch(`${this.baseUrl}/api/v3/votes`, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			credentials: 'omit',
			body: JSON.stringify({
				agid: 1,
				conversation_id: this.polisId,
				tid: votedTid,
				vote: voteValue,
				high_priority: false,
				lang: this.lang,
				...authType
			})
		})
			.then((r) => {
				if (!r.ok) throw new Error(`submitVote failed: ${r.status}`);
				return r.json();
			})
			.then((data) => {
				if (typeof data.currentPid === 'number') {
					this._pid = data.currentPid;
					this.fetchNextStatement();
				}
			})
			.catch((e) => {
				console.error('[PolisApi] Error with vote:', e);
				this._error = e.message;
				this._loading = false;
				this.notify();
			});
	}

	get state(): PolisApiState {
		return {
			currentStatement: this._currentStatement,
			loading: this._loading,
			error: this._error,
			remaining: this._remaining,
			total: this._total,
			ready: this._ready,
			pid: this._pid
		};
	}
}
