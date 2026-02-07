<script lang="ts">
	import type { Session } from '$lib/types';
	import { SessionStatus } from '$lib/types';
	import { getSessionUIState, updateSessionUIState } from '$lib/stores/sessions';

	interface Props {
		session: Session;
		onexpand?: () => void;
		onapprove?: () => void;
		onstop?: () => void;
		onopen?: () => void;
		onsend?: (prompt: string) => void;
	}

	let { session, onexpand, onapprove, onstop, onopen, onsend }: Props = $props();

	let uiState = $derived(getSessionUIState(session.id));
	let quickInput = $state('');

	$effect(() => {
		if (quickInput !== uiState.draftPrompt) {
			updateSessionUIState(session.id, { draftPrompt: quickInput });
		}
	});

	let needsAttention = $derived(
		session.status === SessionStatus.NeedsPermission ||
			session.status === SessionStatus.WaitingForInput
	);

	let isPermission = $derived(session.status === SessionStatus.NeedsPermission);
	let isWaitingInput = $derived(session.status === SessionStatus.WaitingForInput);
	let isWorking = $derived(session.status === SessionStatus.Working);

	let menuOpen = $state(false);

	function getStatusColor(): string {
		switch (session.status) {
			case SessionStatus.Working:
				return 'var(--status-working)';
			case SessionStatus.NeedsPermission:
				return 'var(--status-permission)';
			case SessionStatus.WaitingForInput:
				return 'var(--status-input)';
			case SessionStatus.Connecting:
				return 'var(--status-connecting)';
			default:
				return 'var(--status-connecting)';
		}
	}

	function getStatusLabel(): string {
		switch (session.status) {
			case SessionStatus.Working:
				return 'Working';
			case SessionStatus.NeedsPermission:
				return 'Approval Required';
			case SessionStatus.WaitingForInput:
				return 'Ready';
			case SessionStatus.Connecting:
				return 'Connecting';
			default:
				return 'Unknown';
		}
	}

	function formatTimeSince(isoTimestamp: string): string {
		const now = new Date().getTime();
		const then = new Date(isoTimestamp).getTime();
		const diffMs = now - then;
		const diffMins = Math.floor(diffMs / 60000);
		const diffHours = Math.floor(diffMs / 3600000);
		const diffDays = Math.floor(diffMs / 86400000);

		if (diffMins < 1) return 'now';
		if (diffMins < 60) return `${diffMins}m`;
		if (diffHours < 24) return `${diffHours}h`;
		return `${diffDays}d`;
	}

	function truncateText(text: string, maxLength: number): string {
		if (text.length <= maxLength) return text;
		return text.substring(0, maxLength).trim() + '...';
	}

	function handleCardClick(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (
			target.closest('.action-button') ||
			target.closest('.quick-input') ||
			target.closest('.menu-trigger') ||
			target.closest('.menu-dropdown')
		) {
			return;
		}
		onexpand?.();
	}

	function handleCardKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' || e.key === ' ') {
			const target = e.target as HTMLElement;
			if (target.classList.contains('session-card')) {
				e.preventDefault();
				onexpand?.();
			}
		}
	}

	function handleApprove(e: MouseEvent) {
		e.stopPropagation();
		onapprove?.();
	}

	function handleQuickSend(e: MouseEvent | KeyboardEvent) {
		e.stopPropagation();
		if (quickInput.trim()) {
			onsend?.(quickInput.trim());
			quickInput = '';
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleQuickSend(e);
		}
	}

	function toggleMenu(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = !menuOpen;
	}

	function handleStop(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = false;
		onstop?.();
	}

	function handleOpen(e: MouseEvent) {
		e.stopPropagation();
		menuOpen = false;
		onopen?.();
	}
</script>

<div
	class="session-card"
	class:attention={needsAttention}
	class:permission={isPermission}
	class:waiting={isWaitingInput}
	class:working={isWorking}
	onclick={handleCardClick}
	onkeydown={handleCardKeydown}
	role="button"
	tabindex="0"
>
	<!-- Card Content -->
	<div class="card-body">
		<!-- Header -->
		<div class="card-header">
			<div class="project-info">
				<h3 class="project-name">{session.projectName}</h3>
				{#if session.gitBranch}
					<span class="git-branch">
						<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="6" y1="3" x2="6" y2="15" />
							<circle cx="18" cy="6" r="3" />
							<circle cx="6" cy="18" r="3" />
							<path d="M18 9a9 9 0 0 1-9 9" />
						</svg>
						{session.gitBranch}
					</span>
				{/if}
			</div>
			<div class="header-actions">
				<span class="time-badge">{formatTimeSince(session.modified)}</span>
				<div class="menu-container">
					<button type="button" class="menu-trigger" onclick={toggleMenu} aria-label="More actions">
						<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<circle cx="12" cy="6" r="1.5" fill="currentColor" />
							<circle cx="12" cy="12" r="1.5" fill="currentColor" />
							<circle cx="12" cy="18" r="1.5" fill="currentColor" />
						</svg>
					</button>
					{#if menuOpen}
						<div class="menu-dropdown">
							<button type="button" class="menu-item danger" onclick={handleStop}>
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<rect x="6" y="6" width="12" height="12" rx="1" />
								</svg>
								Stop
							</button>
							<button type="button" class="menu-item" onclick={handleOpen}>
								<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
									<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
									<polyline points="15 3 21 3 21 9" />
									<line x1="10" y1="14" x2="21" y2="3" />
								</svg>
								Open
							</button>
						</div>
					{/if}
				</div>
			</div>
		</div>

		<!-- Status Label -->
		<div class="status-label" style="color: {getStatusColor()}">
			{getStatusLabel()}
		</div>

		<!-- Task Preview -->
		<p class="task-preview">{truncateText(session.firstPrompt, 90)}</p>

		<!-- Footer -->
		<div class="card-footer">
			<span class="message-count">
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
				</svg>
				{session.messageCount}
			</span>
		</div>

		<!-- Action Area -->
		{#if isPermission}
			<div class="action-area">
				<button type="button" class="action-button approve-button" onclick={handleApprove}>
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
						<polyline points="20 6 9 17 4 12" />
					</svg>
					Approve
				</button>
			</div>
		{:else if isWaitingInput}
			<div class="action-area">
				<div class="quick-input-wrapper">
					<input
						type="text"
						class="quick-input"
						placeholder="Quick reply..."
						bind:value={quickInput}
						onkeydown={handleKeydown}
						onclick={(e) => e.stopPropagation()}
					/>
					<button
						type="button"
						class="send-button"
						onclick={handleQuickSend}
						disabled={!quickInput.trim()}
						aria-label="Send message"
					>
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
							<line x1="22" y1="2" x2="11" y2="13" />
							<polygon points="22 2 15 22 11 13 2 9 22 2" />
						</svg>
					</button>
				</div>
			</div>
		{/if}
	</div>
</div>

<style>
	.session-card {
		position: relative;
		display: flex;
		gap: var(--space-lg);
		padding: var(--space-lg) var(--space-xl);
		background: var(--bg-card);
		border: 1px solid var(--border-default);
		border-radius: var(--radius-lg);
		cursor: pointer;
		transition:
			transform var(--transition-normal),
			box-shadow var(--transition-normal),
			border-color var(--transition-normal),
			background var(--transition-fast);
		text-align: left;
		width: 100%;
		height: 100%;
		min-height: 0;
	}

	.session-card::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		background: linear-gradient(135deg, rgba(255, 255, 255, 0.02) 0%, transparent 50%);
		pointer-events: none;
	}

	.session-card:hover {
		background: var(--bg-card-hover);
		border-color: var(--text-muted);
		transform: translateY(-3px);
		box-shadow: var(--shadow-card-hover);
	}

	.session-card.permission {
		border-color: var(--status-permission);
		box-shadow: var(--shadow-glow-amber);
	}

	.session-card.permission:hover {
		box-shadow: var(--shadow-glow-amber), var(--shadow-card-hover);
	}


	/* Card Body */
	.card-body {
		flex: 1;
		min-width: 0;
		display: flex;
		flex-direction: column;
		gap: var(--space-sm);
	}

	.card-header {
		display: flex;
		justify-content: space-between;
		align-items: flex-start;
		gap: var(--space-sm);
	}

	.project-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
		min-width: 0;
	}

	.project-name {
		font-family: var(--font-pixel);
		font-size: 15px;
		font-weight: 500;
		color: var(--text-primary);
		margin: 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		letter-spacing: 0.02em;
	}

	.git-branch {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--text-muted);
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
		flex-shrink: 0;
	}

	.time-badge {
		font-family: var(--font-mono);
		font-size: 11px;
		font-weight: 500;
		color: var(--text-muted);
		background: var(--bg-elevated);
		padding: 2px 8px;
		border-radius: var(--radius-sm);
	}

	/* Status Label */
	.status-label {
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Task Preview */
	.task-preview {
		font-size: 13px;
		color: var(--text-secondary);
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		margin: var(--space-xs) 0;
	}

	/* Footer */
	.card-footer {
		display: flex;
		align-items: center;
		gap: var(--space-md);
		margin-top: auto;
		padding-top: var(--space-sm);
	}

	.message-count {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-family: var(--font-mono);
		font-size: 11px;
		color: var(--text-muted);
	}

	/* Menu */
	.menu-container {
		position: relative;
	}

	.menu-trigger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 28px;
		height: 28px;
		border-radius: var(--radius-sm);
		color: var(--text-muted);
		transition: all var(--transition-fast);
	}

	.menu-trigger:hover {
		background: var(--bg-elevated);
		color: var(--text-primary);
	}

	.menu-dropdown {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 4px;
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		border-radius: var(--radius-md);
		box-shadow: var(--shadow-overlay);
		z-index: 100;
		min-width: 100px;
		padding: var(--space-xs);
		animation: scale-in 0.15s ease;
		transform-origin: top right;
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
		width: 100%;
		padding: var(--space-sm) var(--space-md);
		font-size: 13px;
		font-weight: 500;
		color: var(--text-secondary);
		border-radius: var(--radius-sm);
		transition: all var(--transition-fast);
	}

	.menu-item:hover {
		background: var(--bg-card-hover);
		color: var(--text-primary);
	}

	.menu-item.danger:hover {
		background: rgba(248, 113, 113, 0.1);
		color: var(--accent-red);
	}

	/* Action Area */
	.action-area {
		margin-top: var(--space-md);
		padding-top: var(--space-md);
		border-top: 1px solid var(--border-muted);
	}

	.action-button {
		display: inline-flex;
		align-items: center;
		gap: var(--space-sm);
		padding: var(--space-sm) var(--space-lg);
		font-size: 13px;
		font-weight: 600;
		border-radius: var(--radius-md);
		transition: all var(--transition-fast);
	}

	.approve-button {
		background: var(--status-permission);
		color: #1a1a1a;
	}

	.approve-button:hover {
		filter: brightness(1.1);
		transform: translateY(-1px);
	}

	/* Quick Input */
	.quick-input-wrapper {
		display: flex;
		gap: var(--space-sm);
	}

	.quick-input {
		flex: 1;
		padding: var(--space-sm) var(--space-md);
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		border-radius: var(--radius-md);
		color: var(--text-primary);
		font-size: 13px;
		transition: all var(--transition-fast);
	}

	.quick-input:focus {
		outline: none;
		border-color: var(--status-input);
		box-shadow: 0 0 0 3px var(--status-input-glow);
	}

	.quick-input::placeholder {
		color: var(--text-muted);
	}

	.send-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: var(--status-input);
		color: #1a1a1a;
		border-radius: var(--radius-md);
		transition: all var(--transition-fast);
	}

	.send-button:hover:not(:disabled) {
		filter: brightness(1.1);
		transform: translateY(-1px);
	}

	.send-button:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}
</style>
