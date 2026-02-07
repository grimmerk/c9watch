<script lang="ts">
	import {
		sortedSessions,
		expandedSessionId,
		currentConversation,
		attentionCount
	} from '$lib/stores/sessions';
	import { getConversation, sendPrompt, stopSession, openSession } from '$lib/api';
	import DashboardHeader from '$lib/components/DashboardHeader.svelte';
	import SessionCard from '$lib/components/SessionCard.svelte';
	import ExpandedCardOverlay from '$lib/components/ExpandedCardOverlay.svelte';
	import type { Session } from '$lib/types';

	let sessions = $derived($sortedSessions);
	let expandedId = $derived($expandedSessionId);
	let conversation = $derived($currentConversation);
	let attention = $derived($attentionCount);

	let expandedSession = $derived(sessions.find((s) => s.id === expandedId) || null);

	$effect(() => {
		if (expandedId) {
			getConversation(expandedId)
				.then((conv) => {
					currentConversation.set(conv);
				})
				.catch((error) => {
					console.error('Failed to fetch conversation:', error);
					currentConversation.set(null);
				});
		} else {
			currentConversation.set(null);
		}
	});

	function handleExpand(session: Session) {
		expandedSessionId.set(session.id);
	}

	function handleClose() {
		expandedSessionId.set(null);
	}

	async function handleApprove(sessionId: string) {
		try {
			await sendPrompt(sessionId, 'y');
		} catch (error) {
			console.error('Failed to approve:', error);
		}
	}

	async function handleSend(sessionId: string, prompt: string) {
		try {
			await sendPrompt(sessionId, prompt);
		} catch (error) {
			console.error('Failed to send prompt:', error);
		}
	}

	async function handleStop(pid: number) {
		try {
			await stopSession(pid);
		} catch (error) {
			console.error('Failed to stop session:', error);
		}
	}

	async function handleOpen(pid: number, projectPath: string) {
		try {
			await openSession(pid, projectPath);
		} catch (error) {
			console.error('Failed to open session:', error);
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key >= '1' && e.key <= '9' && !expandedId) {
			const index = parseInt(e.key) - 1;
			if (index < sessions.length) {
				handleExpand(sessions[index]);
			}
		}
		if (e.key === 'Tab' && !expandedId && attention > 0) {
			e.preventDefault();
			const attentionSessions = sessions.filter(
				(s) => s.status === 'NeedsPermission' || s.status === 'WaitingForInput'
			);
			if (attentionSessions.length > 0) {
				handleExpand(attentionSessions[0]);
			}
		}
	}
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="dashboard">
	<DashboardHeader />

	<main class="grid-container">
		{#if sessions.length === 0}
			<div class="empty-state">
				<div class="empty-visual">
					<div class="empty-orb">
						<div class="orb-core"></div>
						<div class="orb-ring ring-1"></div>
						<div class="orb-ring ring-2"></div>
						<div class="orb-ring ring-3"></div>
					</div>
				</div>
				<div class="empty-content">
					<h2>No Active Sessions</h2>
					<p>Start a Claude Code session in your terminal or IDE</p>
					<div class="empty-hint">
						<span class="hint-icon">
							<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
								<circle cx="12" cy="12" r="10" />
								<path d="M12 16v-4" />
								<path d="M12 8h.01" />
							</svg>
						</span>
						Sessions are detected automatically
					</div>
				</div>
			</div>
		{:else}
			<div class="session-grid">
				{#each sessions as session, index (session.id)}
					<div class="grid-item" style="--item-index: {index}">
						<SessionCard
							{session}
							onexpand={() => handleExpand(session)}
							onapprove={() => handleApprove(session.id)}
							onsend={(prompt) => handleSend(session.id, prompt)}
							onstop={() => handleStop(session.pid)}
							onopen={() => handleOpen(session.pid, session.projectPath)}
						/>
					</div>
				{/each}
			</div>
		{/if}
	</main>

	{#if expandedSession}
		<ExpandedCardOverlay
			session={expandedSession}
			{conversation}
			onclose={handleClose}
			onsend={(prompt) => handleSend(expandedSession.id, prompt)}
			onstop={() => handleStop(expandedSession.pid)}
			onopen={() => handleOpen(expandedSession.pid, expandedSession.projectPath)}
			onapprove={() => handleApprove(expandedSession.id)}
		/>
	{/if}

	{#if expandedId && attention > 0}
		<button type="button" class="attention-float" onclick={handleClose}>
			<span class="attention-pulse"></span>
			<span class="attention-count">{attention}</span>
			<span class="attention-text">
				{attention === 1 ? 'session needs' : 'sessions need'} attention
			</span>
			<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<polyline points="9 18 15 12 9 6" />
			</svg>
		</button>
	{/if}
</div>

<style>
	.dashboard {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
		overflow: hidden;
		background: var(--bg-base);
	}

	.grid-container {
		flex: 1;
		overflow-y: auto;
		padding: var(--space-xl);
	}

	.session-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
		grid-auto-rows: 1fr;
		gap: var(--space-lg);
		max-width: 1440px;
		margin: 0 auto;
	}

	.grid-item {
		display: flex;
		animation: slide-up 0.4s cubic-bezier(0.16, 1, 0.3, 1) backwards;
		animation-delay: calc(var(--item-index) * 60ms);
	}

	/* Empty State */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		height: 70vh;
		text-align: center;
		gap: var(--space-2xl);
	}

	.empty-visual {
		position: relative;
		width: 120px;
		height: 120px;
	}

	.empty-orb {
		position: relative;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.orb-core {
		width: 24px;
		height: 24px;
		border-radius: 50%;
		background: var(--accent-blue);
		box-shadow: 0 0 40px var(--status-working-glow);
		animation: pulse-glow 3s ease-in-out infinite;
	}

	.orb-ring {
		position: absolute;
		border-radius: 50%;
		border: 1px solid var(--accent-blue);
		opacity: 0.3;
	}

	.ring-1 {
		inset: 20px;
		animation: ring-pulse 3s ease-out infinite;
	}

	.ring-2 {
		inset: 10px;
		animation: ring-pulse 3s ease-out infinite;
		animation-delay: 1s;
	}

	.ring-3 {
		inset: 0;
		animation: ring-pulse 3s ease-out infinite;
		animation-delay: 2s;
	}

	.empty-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--space-md);
	}

	.empty-content h2 {
		font-size: 24px;
		font-weight: 600;
		color: var(--text-primary);
		letter-spacing: -0.02em;
	}

	.empty-content p {
		font-size: 15px;
		color: var(--text-secondary);
	}

	.empty-hint {
		display: inline-flex;
		align-items: center;
		gap: var(--space-sm);
		margin-top: var(--space-md);
		padding: var(--space-sm) var(--space-lg);
		background: var(--bg-card);
		border: 1px solid var(--border-default);
		border-radius: var(--radius-lg);
		font-size: 13px;
		color: var(--text-muted);
	}

	.hint-icon {
		display: flex;
		color: var(--accent-blue);
	}

	/* Attention Float */
	.attention-float {
		position: fixed;
		bottom: var(--space-xl);
		right: var(--space-xl);
		display: flex;
		align-items: center;
		gap: var(--space-sm);
		padding: var(--space-md) var(--space-lg);
		background: linear-gradient(135deg, var(--status-permission), #ea580c);
		color: white;
		border-radius: var(--radius-xl);
		font-size: 13px;
		font-weight: 600;
		box-shadow: 0 4px 24px var(--status-permission-glow), 0 0 0 1px rgba(255, 255, 255, 0.1) inset;
		z-index: 999;
		animation: slide-up 0.3s ease;
		transition: all var(--transition-fast);
		overflow: hidden;
	}

	.attention-float:hover {
		transform: translateY(-2px) scale(1.02);
		box-shadow: 0 8px 32px var(--status-permission-glow), 0 0 0 1px rgba(255, 255, 255, 0.15) inset;
	}

	.attention-pulse {
		position: absolute;
		inset: 0;
		background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
		animation: shimmer 2s ease-in-out infinite;
	}

	.attention-count {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		min-width: 24px;
		height: 24px;
		padding: 0 6px;
		background: rgba(0, 0, 0, 0.2);
		border-radius: var(--radius-md);
		font-family: var(--font-mono);
		font-size: 12px;
	}

	.attention-text {
		position: relative;
		white-space: nowrap;
	}
</style>
