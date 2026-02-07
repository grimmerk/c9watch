<script lang="ts">
	import type { Message } from '$lib/types';

	interface Props {
		message: Message;
	}

	let { message }: Props = $props();

	function formatTime(isoTimestamp: string): string {
		const date = new Date(isoTimestamp);
		return date.toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	let isUser = $derived(message.messageType === 'User');
	let isAssistant = $derived(message.messageType === 'Assistant');
	let isThinking = $derived(message.messageType === 'Thinking');
	let isToolUse = $derived(message.messageType === 'ToolUse');
	let isToolResult = $derived(message.messageType === 'ToolResult');

	let roleLabel = $derived.by(() => {
		switch (message.messageType) {
			case 'User':
				return 'You';
			case 'Assistant':
				return 'Claude';
			case 'Thinking':
				return 'Thinking';
			case 'ToolUse':
				return 'Tool';
			case 'ToolResult':
				return 'Result';
			default:
				return 'Unknown';
		}
	});

	let roleIcon = $derived.by(() => {
		switch (message.messageType) {
			case 'User':
				return '→';
			case 'Assistant':
				return '◆';
			case 'Thinking':
				return '◇';
			case 'ToolUse':
				return '⚙';
			case 'ToolResult':
				return '↩';
			default:
				return '•';
		}
	});
</script>

<div
	class="message-bubble"
	class:user={isUser}
	class:assistant={isAssistant}
	class:thinking={isThinking}
	class:tool-use={isToolUse}
	class:tool-result={isToolResult}
>
	<div class="message-header">
		<span class="message-icon">{roleIcon}</span>
		<span class="message-role">{roleLabel}</span>
		<span class="message-time">{formatTime(message.timestamp)}</span>
	</div>

	{#if message.content}
		<div class="message-content">{message.content}</div>
	{/if}
</div>

<style>
	.message-bubble {
		margin: var(--space-sm) 0;
		padding: var(--space-md) var(--space-lg);
		border-radius: var(--radius-md);
		max-width: 90%;
		border-left: 3px solid transparent;
	}

	.message-bubble.user {
		background: rgba(88, 166, 255, 0.1);
		border-left-color: var(--accent-blue);
		margin-left: auto;
	}

	.message-bubble.assistant {
		background: var(--bg-card);
		border-left-color: var(--text-muted);
	}

	.message-bubble.thinking {
		background: rgba(240, 136, 62, 0.1);
		border-left-color: var(--accent-amber);
		font-style: italic;
	}

	.message-bubble.tool-use {
		background: rgba(63, 185, 80, 0.08);
		border-left-color: var(--accent-green);
		font-family: var(--font-mono);
		font-size: 12px;
	}

	.message-bubble.tool-result {
		background: var(--bg-card);
		border-left-color: var(--text-muted);
		font-family: var(--font-mono);
		font-size: 12px;
		opacity: 0.8;
	}

	.message-header {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
		margin-bottom: var(--space-sm);
	}

	.message-icon {
		font-size: 12px;
		opacity: 0.6;
	}

	.message-role {
		font-weight: 600;
		font-size: 12px;
		color: var(--text-secondary);
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.message-bubble.user .message-role {
		color: var(--accent-blue);
	}

	.message-bubble.thinking .message-role {
		color: var(--accent-amber);
	}

	.message-bubble.tool-use .message-role {
		color: var(--accent-green);
	}

	.message-time {
		margin-left: auto;
		font-size: 11px;
		color: var(--text-muted);
		font-family: var(--font-mono);
	}

	.message-content {
		color: var(--text-primary);
		font-size: 14px;
		line-height: 1.6;
		white-space: pre-wrap;
		word-wrap: break-word;
	}

	.message-bubble.tool-use .message-content,
	.message-bubble.tool-result .message-content {
		color: var(--text-secondary);
		max-height: 200px;
		overflow-y: auto;
	}
</style>
