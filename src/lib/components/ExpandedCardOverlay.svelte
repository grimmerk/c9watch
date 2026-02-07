<script lang="ts">
	import { onMount, tick } from 'svelte';
	import type { Session, Conversation } from '$lib/types';
	import { SessionStatus } from '$lib/types';
	import { getSessionUIState, updateSessionUIState } from '$lib/stores/sessions';
	import MessageBubble from './MessageBubble.svelte';

	interface Props {
		session: Session;
		conversation: Conversation | null;
		onclose?: () => void;
		onsend?: (prompt: string) => void;
		onstop?: () => void;
		onopen?: () => void;
		onapprove?: () => void;
	}

	let { session, conversation, onclose, onsend, onstop, onopen, onapprove }: Props = $props();

	let uiState = $derived(getSessionUIState(session.id));
	let promptInput = $state('');
	let messagesContainer: HTMLDivElement;
	let isInitialLoad = $state(true);

	onMount(() => {
		if (messagesContainer && uiState.scrollPosition > 0) {
			messagesContainer.scrollTop = uiState.scrollPosition;
		}
		isInitialLoad = false;

		const handleKeydown = (e: KeyboardEvent) => {
			if (e.key === 'Escape') {
				handleClose();
			}
		};
		window.addEventListener('keydown', handleKeydown);
		return () => window.removeEventListener('keydown', handleKeydown);
	});

	function handleScroll() {
		if (messagesContainer && !isInitialLoad) {
			updateSessionUIState(session.id, { scrollPosition: messagesContainer.scrollTop });
		}
	}

	$effect(() => {
		if (conversation && conversation.messages.length > 0 && messagesContainer && !isInitialLoad) {
			const isAtBottom =
				messagesContainer.scrollHeight - messagesContainer.scrollTop - messagesContainer.clientHeight < 100;
			if (isAtBottom) {
				tick().then(() => {
					messagesContainer.scrollTop = messagesContainer.scrollHeight;
				});
			}
		}
	});

	$effect(() => {
		if (promptInput !== uiState.draftPrompt) {
			updateSessionUIState(session.id, { draftPrompt: promptInput });
		}
	});

	let isPermission = $derived(session.status === SessionStatus.NeedsPermission);
	let isWaitingInput = $derived(session.status === SessionStatus.WaitingForInput);
	let isWorking = $derived(session.status === SessionStatus.Working);

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

	function handleClose() {
		if (messagesContainer) {
			updateSessionUIState(session.id, { scrollPosition: messagesContainer.scrollTop });
		}
		onclose?.();
	}

	function handleBackdropClick(e: MouseEvent) {
		if (e.target === e.currentTarget) {
			handleClose();
		}
	}

	function handleSend() {
		if (promptInput.trim()) {
			onsend?.(promptInput.trim());
			promptInput = '';
			updateSessionUIState(session.id, { draftPrompt: '' });
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSend();
		}
	}
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div class="overlay-backdrop" onclick={handleBackdropClick} role="dialog" aria-modal="true" aria-labelledby="overlay-title" tabindex="-1">
	<div class="overlay-card" class:permission={isPermission} class:waiting={isWaitingInput}>
		<!-- Accent bar -->
		<div class="accent-bar" style="background: {getStatusColor()}"></div>

		<!-- Header -->
		<header class="overlay-header">
			<div class="header-left">
				<div class="status-indicator" style="--status-color: {getStatusColor()}">
					<div class="status-dot"></div>
					{#if isPermission || isWaitingInput}
						<div class="status-ring"></div>
					{/if}
				</div>
				<div class="header-info">
					<div class="header-title">
						<h2 id="overlay-title" class="project-name">{session.projectName}</h2>
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
					<div class="header-meta">
						<span class="status-label" style="color: {getStatusColor()}">{getStatusLabel()}</span>
						<span class="separator">·</span>
						<span class="message-count">{conversation?.messages.length ?? 0} messages</span>
					</div>
				</div>
			</div>
			<div class="header-actions">
				<button type="button" class="header-button" onclick={() => onstop?.()} title="Stop Session">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<rect x="6" y="6" width="12" height="12" rx="1" />
					</svg>
				</button>
				<button type="button" class="header-button" onclick={() => onopen?.()} title="Open in IDE">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
						<polyline points="15 3 21 3 21 9" />
						<line x1="10" y1="14" x2="21" y2="3" />
					</svg>
				</button>
				<div class="header-divider"></div>
				<button type="button" class="close-button" onclick={handleClose} aria-label="Close">
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
						<line x1="18" y1="6" x2="6" y2="18" />
						<line x1="6" y1="6" x2="18" y2="18" />
					</svg>
				</button>
			</div>
		</header>

		<!-- Conversation Area -->
		<div class="conversation-area" bind:this={messagesContainer} onscroll={handleScroll}>
			{#if !conversation}
				<div class="loading-state">
					<div class="loading-spinner"></div>
					<p>Loading conversation...</p>
				</div>
			{:else if conversation.messages.length === 0}
				<div class="empty-state">
					<div class="empty-icon">
						<svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
							<path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
						</svg>
					</div>
					<p>No messages yet</p>
					<p class="empty-hint">Send a message to start the conversation</p>
				</div>
			{:else}
				<div class="messages">
					{#each conversation.messages as message, index (index)}
						<MessageBubble {message} />
					{/each}
				</div>
			{/if}
		</div>

		<!-- Input Area -->
		<footer class="input-area">
			{#if isPermission}
				<button type="button" class="approve-button" onclick={() => onapprove?.()}>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
						<polyline points="20 6 9 17 4 12" />
					</svg>
					Approve Permission Request
				</button>
			{:else}
				<div class="input-wrapper">
					<textarea
						class="prompt-textarea"
						placeholder={isWaitingInput ? 'Type your message...' : 'Session is working...'}
						bind:value={promptInput}
						onkeydown={handleKeydown}
						rows="1"
						disabled={!isWaitingInput}
					></textarea>
					<button
						type="button"
						class="send-button"
						onclick={handleSend}
						disabled={!promptInput.trim() || !isWaitingInput}
						aria-label="Send message"
					>
						<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<line x1="22" y1="2" x2="11" y2="13" />
							<polygon points="22 2 15 22 11 13 2 9 22 2" />
						</svg>
					</button>
				</div>
			{/if}
		</footer>
	</div>
</div>

<style>
	.overlay-backdrop {
		position: fixed;
		inset: 0;
		background: var(--bg-overlay);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		display: flex;
		align-items: center;
		justify-content: center;
		z-index: 1000;
		animation: fade-in 0.2s ease;
		padding: var(--space-2xl);
	}

	.overlay-card {
		position: relative;
		width: 100%;
		max-width: 820px;
		height: 85vh;
		max-height: 920px;
		background: var(--bg-elevated);
		border: 1px solid var(--border-default);
		border-radius: var(--radius-2xl);
		box-shadow: var(--shadow-overlay);
		display: flex;
		flex-direction: column;
		animation: slide-up 0.35s cubic-bezier(0.16, 1, 0.3, 1);
		overflow: hidden;
	}

	.overlay-card::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: inherit;
		background: linear-gradient(180deg, rgba(255, 255, 255, 0.02) 0%, transparent 20%);
		pointer-events: none;
	}

	/* Accent Bar */
	.accent-bar {
		height: 3px;
		flex-shrink: 0;
	}

	/* Header */
	.overlay-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: var(--space-lg) var(--space-xl);
		border-bottom: 1px solid var(--border-default);
		background: var(--bg-card);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: var(--space-md);
	}

	.status-indicator {
		position: relative;
		width: 14px;
		height: 14px;
		flex-shrink: 0;
	}

	.status-dot {
		width: 100%;
		height: 100%;
		border-radius: 50%;
		background: var(--status-color);
		box-shadow: 0 0 12px var(--status-color);
	}

	.status-ring {
		position: absolute;
		inset: -3px;
		border-radius: 50%;
		border: 2px solid var(--status-color);
		animation: ring-pulse 2s ease-out infinite;
	}

	.header-info {
		display: flex;
		flex-direction: column;
		gap: 2px;
	}

	.header-title {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
	}

	.project-name {
		font-size: 18px;
		font-weight: 600;
		color: var(--text-primary);
		margin: 0;
		letter-spacing: -0.01em;
	}

	.git-branch {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		font-family: var(--font-mono);
		font-size: 12px;
		color: var(--text-muted);
		background: var(--bg-base);
		padding: 2px 8px;
		border-radius: var(--radius-sm);
	}

	.header-meta {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
		font-size: 12px;
	}

	.status-label {
		font-weight: 600;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.separator {
		color: var(--text-muted);
	}

	.message-count {
		color: var(--text-muted);
		font-family: var(--font-mono);
	}

	.header-actions {
		display: flex;
		align-items: center;
		gap: var(--space-xs);
	}

	.header-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border-radius: var(--radius-md);
		color: var(--text-muted);
		transition: all var(--transition-fast);
	}

	.header-button:hover {
		background: var(--bg-base);
		color: var(--text-primary);
	}

	.header-divider {
		width: 1px;
		height: 20px;
		background: var(--border-default);
		margin: 0 var(--space-sm);
	}

	.close-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		border-radius: var(--radius-md);
		color: var(--text-muted);
		transition: all var(--transition-fast);
	}

	.close-button:hover {
		background: rgba(248, 113, 113, 0.15);
		color: var(--accent-red);
	}

	/* Conversation Area */
	.conversation-area {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-xl);
	}

	.messages {
		display: flex;
		flex-direction: column;
	}

	.loading-state,
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 100%;
		gap: var(--space-md);
		color: var(--text-muted);
	}

	.loading-spinner {
		width: 32px;
		height: 32px;
		border: 3px solid var(--border-default);
		border-top-color: var(--accent-blue);
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	.empty-icon {
		opacity: 0.3;
		margin-bottom: var(--space-sm);
	}

	.empty-hint {
		font-size: 13px;
		color: var(--text-muted);
	}

	/* Input Area */
	.input-area {
		padding: var(--space-lg) var(--space-xl);
		border-top: 1px solid var(--border-default);
		background: var(--bg-card);
	}

	.input-wrapper {
		display: flex;
		gap: var(--space-md);
		align-items: flex-end;
	}

	.prompt-textarea {
		flex: 1;
		padding: var(--space-md) var(--space-lg);
		background: var(--bg-base);
		border: 1px solid var(--border-default);
		border-radius: var(--radius-lg);
		color: var(--text-primary);
		font-size: 14px;
		resize: none;
		min-height: 48px;
		max-height: 140px;
		transition: all var(--transition-fast);
	}

	.prompt-textarea:focus {
		outline: none;
		border-color: var(--accent-blue);
		box-shadow: 0 0 0 3px var(--border-glow);
	}

	.prompt-textarea:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.prompt-textarea::placeholder {
		color: var(--text-muted);
	}

	.send-button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 48px;
		height: 48px;
		background: var(--accent-blue);
		color: #1a1a1a;
		border-radius: var(--radius-lg);
		transition: all var(--transition-fast);
		flex-shrink: 0;
	}

	.send-button:hover:not(:disabled) {
		filter: brightness(1.1);
		transform: translateY(-1px);
	}

	.send-button:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	.approve-button {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-sm);
		width: 100%;
		padding: var(--space-md) var(--space-xl);
		background: linear-gradient(135deg, var(--status-permission), #ea580c);
		color: white;
		font-size: 15px;
		font-weight: 600;
		border-radius: var(--radius-lg);
		transition: all var(--transition-fast);
		box-shadow: 0 2px 12px var(--status-permission-glow);
	}

	.approve-button:hover {
		filter: brightness(1.08);
		transform: translateY(-1px);
		box-shadow: 0 4px 20px var(--status-permission-glow);
	}
</style>
