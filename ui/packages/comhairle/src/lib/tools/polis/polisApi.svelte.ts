export type PolisStatement = {
	txt: string,
	tid: number,
	is_meta: boolean,
	is_seen: boolean,
	lang: string | undefined,
	remaining: number,
	total: number
}

export default class PolisApi {
	_currentStatement = $state<PolisStatement | undefined>();
	_loading = $state<boolean>(false)
	_remaining = $state<number>(0)
	_total = $state<number>(0)

	polisId: string;
	userId: string;
	lang: string;
	baseUrl: string;
	pid: number | undefined = undefined;

	constructor(userId: string, polisId: string, lang: string = "en", baseUrl: string = "https://polis.comhairle.scot") {
		this.polisId = polisId
		this.userId = userId
		this.lang = lang
		this.baseUrl = baseUrl
		this.getOrCreateParticipant().then((pid) => {
			this.pid = pid
			this.fetchNextStatement()
		})
	}

	async getOrCreateParticipant() {
		// Step 1: Check if participant already exists
		const checkResponse = await fetch(
			`${this.baseUrl}/api/v3/participants?conversation_id=${this.polisId}&xid=${this.userId}`,
			{
				method: 'GET',
				credentials: 'omit'
			}
		);

		const existing = await checkResponse.json();

		// Step 2: If exists, return the pid
		if (existing && existing.pid) {
			console.log('Participant already exists:', existing);
			return existing.pid;
		}

		// Step 3: If not exists (null), create new participant
		console.log('Participant not found, creating...');
		const createResponse = await fetch(
			`${this.baseUrl}/api/v3/participants`,
			{
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				credentials: 'omit',
				body: JSON.stringify({
					conversation_id: this.polisId,
					xid: this.userId
				})
			}
		);

		const created = await createResponse.json();
		console.log('Participant created:', created);
		return created.pid;
	}

	fetchNextStatement() {
		if (this.pid) {
			this._loading = true
			fetch(`${this.baseUrl}/api/v3/nextComment?conversation_id=${this.polisId}&not_voted_by_pid=${this.pid}`, { credentials: 'include' })
				.then((s) => s.json())
				.then((comment) => {
					console.log("loaded statement comment ", comment)
					if (comment.txt) {
						this._currentStatement = comment
						this._remaining = comment.remaining
						this._total = comment.total
					}
				})
				.finally(() => this._loading = false)
		}
		else {
			console.error("Trying to fetch statement without pid")
		}
	}

	submitStatement(statement: string) {
		fetch('https://polis.comhairle.scot/api/v3/comments', {
			method: 'POST',
			credentials: 'omit',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({
				conversation_id: this.polisId,
				txt: statement,  // Max 997 characters
				pid: this.pid,
				xid: this.userId,  // Your user XID
				vote: -1,  // Optional: auto-vote on your own comment (1=agree, -1=disagree, 0=pass)
				// anon: false,  // Optional: anonymous comment
				// is_seed: false  // Optional: mark as seed comment
			})
		}).then(r => r.json())
			.then(data => {
				console.log('Comment created:', data);
				// Response includes: tid (comment ID), txt, pid, created timestamp, etc.
			})
			.catch(err => console.error('Error:', err));
	}


	submitVote(vote: "agree" | "disagree" | "pass") {
		if (this.pid) {
			this._loading = true
			let voteValue = { "agree": -1, "disagree": 1, "pass": 0 }[vote]
			if (this.currentStatement) {

				fetch(`${this.baseUrl}/api/v3/votes`, {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					credentials: 'include',
					body: JSON.stringify({
						agid: 1,
						conversation_id: this.polisId,
						tid: this.currentStatement.tid,              // The statement ID
						vote: voteValue,              // -1 = agree, 0 = pass, 1 = disagree
						pid: this.pid,
						high_priority: false,
						lang: this.lang,
					})
				})
					.then(r => r.json())
					.then(data => {
						console.log('Current participant:', data);
						this._currentStatement = data.nextComment
					});
			}
			else {
				console.error("No current statement to vote on")
			}
		}
		else {
			console.error("Trying to send vote without a pid")
		}
	}

	get currentStatement() {
		return this._currentStatement
	}

	get loading() {
		return this._loading
	}

	get remaining() {
		return this._remaining
	}

	get total() {
		return this._total
	}

} 
