<script lang="ts">
	import type { Session } from '$lib/types';
	import { SessionStatus } from '$lib/types';
	import { getSessionUIState, updateSessionUIState } from '$lib/stores/sessions';
	import { invoke } from '@tauri-apps/api/core';

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
	let isEditingName = $state(false);
	let tempName = $state(session.projectName);

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


	function handleCardClick(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (
			target.closest('.action-button') ||
			target.closest('.quick-input') ||
			target.closest('.menu-trigger') ||
			target.closest('.menu-dropdown') ||
			target.closest('.project-name-input')
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

	async function saveName() {
		if (tempName.trim() && tempName !== session.projectName) {
			try {
				await invoke('rename_session', { sessionId: session.id, newName: tempName.trim() });
			} catch (err) {
				console.error('Failed to rename session:', err);
				tempName = session.projectName;
			}
		}
		isEditingName = false;
	}

	function handleNameKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter') {
			saveName();
		} else if (e.key === 'Escape') {
			tempName = session.projectName;
			isEditingName = false;
		}
	}

	function startEditing(e: MouseEvent) {
		e.stopPropagation();
		tempName = session.projectName;
		isEditingName = true;
		menuOpen = false;
	}

	function autofocus(node: HTMLElement) {
		node.focus();
		if (node instanceof HTMLInputElement) {
			node.select();
		}
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
			{#if isEditingName}
				<input
					type="text"
					class="project-name-input"
					bind:value={tempName}
					onkeydown={handleNameKeydown}
					onblur={saveName}
					use:autofocus
					onclick={(e) => e.stopPropagation()}
				/>
			{:else}
				<h3 class="project-name" ondblclick={startEditing}>{session.projectName}</h3>
			{/if}
			<span class="message-count">
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
				</svg>
				{session.messageCount}
			</span>
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
						<button type="button" class="menu-item" onclick={startEditing}>
							<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
								<path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
							</svg>
							Rename
						</button>
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

		<!-- Git Branch -->
		{#if session.gitBranch}
			<div class="git-branch">
				<svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
					<line x1="6" y1="3" x2="6" y2="15" />
					<circle cx="18" cy="6" r="3" />
					<circle cx="6" cy="18" r="3" />
					<path d="M18 9a9 9 0 0 1-9 9" />
				</svg>
				<span class="branch-name">{session.gitBranch}</span>
			</div>
		{/if}

		<!-- Status Label -->
		<div class="status-label" style="color: {getStatusColor()}">
			{getStatusLabel()}
		</div>

		<!-- Task Preview -->
		<!-- Task Preview -->
		<p class="task-preview">{session.firstPrompt}</p>

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
		padding: var(--space-lg);
		background: var(--bg-card);
		border: 1px solid var(--border-default);
		cursor: pointer;
		transition: border-color var(--transition-fast);
		text-align: left;
		width: 100%;
		height: 235px;
	}

	.session-card:hover {
		border-color: var(--text-muted);
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
		align-items: center;
		gap: var(--space-sm);
	}

	.project-name {
		font-family: var(--font-pixel);
		font-size: 14px;
		font-weight: 500;
		color: var(--text-primary);
		margin: 0;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		letter-spacing: 0.05em;
		text-transform: uppercase;
		flex: 1;
		min-width: 0;
	}

	.project-name-input {
		font-family: var(--font-pixel);
		font-size: 14px;
		font-weight: 500;
		color: var(--text-primary);
		background: var(--bg-base);
		border: 1px solid var(--status-input);
		padding: 2px 4px;
		margin: -3px -5px;
		letter-spacing: 0.05em;
		text-transform: uppercase;
		flex: 1;
		min-width: 0;
		outline: none;
	}

	.git-branch {
		display: flex;
		align-items: center;
		gap: 4px;
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--text-muted);
		text-transform: lowercase;
		min-width: 0;
	}

	.git-branch svg {
		flex-shrink: 0;
	}

	.branch-name {
		overflow: hidden;
		white-space: nowrap;
		text-overflow: ellipsis;
		min-width: 0;
		max-width: 200px;
	}

	.time-badge {
		font-family: var(--font-mono);
		font-size: 10px;
		font-weight: 500;
		color: var(--text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Status Label */
	.status-label {
		font-family: var(--font-mono);
		font-size: 10px;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.1em;
	}

	/* Task Preview */
	.task-preview {
		font-size: 12px;
		color: var(--text-secondary);
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		margin: var(--space-xs) 0;
	}

	.message-count {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-family: var(--font-mono);
		font-size: 10px;
		color: var(--text-muted);
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	/* Menu */
	.menu-container {
		position: relative;
	}

	.menu-trigger {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 24px;
		height: 24px;
		color: var(--text-muted);
		transition: color var(--transition-fast);
	}

	.menu-trigger:hover {
		color: var(--text-primary);
	}

	.menu-dropdown {
		position: absolute;
		top: 100%;
		right: 0;
		margin-top: 4px;
		background: var(--bg-card);
		border: 1px solid var(--border-default);
		z-index: 100;
		min-width: 100px;
		padding: var(--space-xs);
	}

	.menu-item {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
		width: 100%;
		padding: var(--space-sm) var(--space-md);
		font-family: var(--font-mono);
		font-size: 11px;
		font-weight: 500;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.05em;
		transition: color var(--transition-fast);
	}

	.menu-item:hover {
		color: var(--text-primary);
	}

	.menu-item.danger:hover {
		color: var(--accent-red);
	}

	.action-area {
		margin-top: auto;
		padding-top: var(--space-md);
		border-top: 1px solid var(--border-muted);
	}

	.action-button {
		display: inline-flex;
		align-items: center;
		gap: var(--space-sm);
		padding: var(--space-sm) var(--space-lg);
		font-family: var(--font-mono);
		font-size: 11px;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border: 1px solid transparent;
		transition: all var(--transition-fast);
	}

	.approve-button {
		background: var(--status-permission);
		color: var(--bg-base);
		border-color: var(--status-permission);
	}

	.approve-button:hover {
		background: transparent;
		color: var(--status-permission);
	}

	/* Quick Input */
	.quick-input-wrapper {
		display: flex;
		gap: var(--space-sm);
	}

	.quick-input {
		flex: 1;
		padding: var(--space-sm) var(--space-md);
		background: var(--bg-base);
		border: 1px solid var(--border-default);
		color: var(--text-primary);
		font-family: var(--font-mono);
		font-size: 12px;
		transition: border-color var(--transition-fast);
	}

	.quick-input:focus {
		outline: none;
		border-color: var(--status-input);
	}

	.quick-input::placeholder {
		color: var(--text-muted);
	}

	.send-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 32px;
		height: 32px;
		background: var(--status-input);
		color: var(--bg-base);
		border: 1px solid var(--status-input);
		transition: all var(--transition-fast);
	}

	.send-button:hover:not(:disabled) {
		background: transparent;
		color: var(--status-input);
	}

	.send-button:disabled {
		opacity: 0.3;
		cursor: not-allowed;
	}
</style>
