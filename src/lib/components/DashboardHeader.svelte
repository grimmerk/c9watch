<script lang="ts">
	import { sessions, statusSummary, attentionCount } from '$lib/stores/sessions';

	let sessionCount = $derived($sessions.length);
	let summary = $derived($statusSummary);
	let needsAttention = $derived($attentionCount);
</script>

<header class="dashboard-header">
	<!-- Accent line -->
	<div class="accent-line"></div>

	<div class="header-content">
		<div class="header-left">
			<div class="title-group">
				<span class="session-count">{sessionCount}</span>
				<div class="title-text">
					<span class="title-main">Active {sessionCount === 1 ? 'Session' : 'Sessions'}</span>
					<span class="title-sub">Claude Code Monitor</span>
				</div>
			</div>
			{#if needsAttention > 0}
				<div class="attention-badge">
					<span class="attention-icon">
						<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
							<circle cx="12" cy="12" r="10" />
							<line x1="12" y1="8" x2="12" y2="12" />
							<line x1="12" y1="16" x2="12.01" y2="16" />
						</svg>
					</span>
					{needsAttention} need{needsAttention === 1 ? 's' : ''} attention
				</div>
			{/if}
		</div>

		<div class="header-right">
			<div class="status-summary">
				{#if summary.working > 0}
					<div class="status-pill working">
						<span class="status-indicator"></span>
						<span class="status-count">{summary.working}</span>
						<span class="status-label">working</span>
					</div>
				{/if}
				{#if summary.permission > 0}
					<div class="status-pill permission">
						<span class="status-indicator"></span>
						<span class="status-count">{summary.permission}</span>
						<span class="status-label">permission</span>
					</div>
				{/if}
				{#if summary.input > 0}
					<div class="status-pill input">
						<span class="status-indicator"></span>
						<span class="status-count">{summary.input}</span>
						<span class="status-label">waiting</span>
					</div>
				{/if}
				{#if summary.connecting > 0}
					<div class="status-pill connecting">
						<span class="status-indicator"></span>
						<span class="status-count">{summary.connecting}</span>
						<span class="status-label">connecting</span>
					</div>
				{/if}
			</div>
			<div class="divider"></div>
			<div class="live-indicator">
				<span class="live-dot"></span>
				<span class="live-text">LIVE</span>
			</div>
		</div>
	</div>
</header>

<style>
	.dashboard-header {
		position: relative;
		background: var(--bg-elevated);
		border-bottom: 1px solid var(--border-default);
	}

	.accent-line {
		height: 2px;
		background: linear-gradient(
			90deg,
			var(--accent-blue) 0%,
			var(--accent-purple) 50%,
			var(--accent-blue) 100%
		);
		opacity: 0.6;
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: var(--space-lg) var(--space-xl);
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: var(--space-xl);
	}

	.title-group {
		display: flex;
		align-items: center;
		gap: var(--space-md);
	}

	.session-count {
		font-family: var(--font-pixel);
		font-size: 36px;
		font-weight: 500;
		color: var(--accent-blue);
		line-height: 1;
		letter-spacing: 0.02em;
	}

	.title-text {
		display: flex;
		flex-direction: column;
		gap: 0;
	}

	.title-main {
		font-size: 16px;
		font-weight: 600;
		color: var(--text-primary);
		line-height: 1.2;
	}

	.title-sub {
		font-family: var(--font-pixel);
		font-size: 11px;
		font-weight: 500;
		color: var(--text-muted);
		text-transform: uppercase;
		letter-spacing: 0.15em;
	}

	.attention-badge {
		display: inline-flex;
		align-items: center;
		gap: var(--space-sm);
		padding: var(--space-sm) var(--space-md);
		background: linear-gradient(135deg, var(--status-permission), #ea580c);
		color: white;
		font-size: 12px;
		font-weight: 600;
		border-radius: var(--radius-md);
		box-shadow: 0 2px 8px var(--status-permission-glow);
		animation: pulse-glow 2.5s ease-in-out infinite;
	}

	.attention-icon {
		display: flex;
		opacity: 0.9;
	}

	.header-right {
		display: flex;
		align-items: center;
		gap: var(--space-lg);
	}

	.status-summary {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
	}

	.status-pill {
		display: flex;
		align-items: center;
		gap: var(--space-xs);
		padding: var(--space-xs) var(--space-md);
		background: var(--bg-card);
		border: 1px solid var(--border-default);
		border-radius: var(--radius-lg);
		font-size: 12px;
	}

	.status-indicator {
		width: 8px;
		height: 8px;
		border-radius: 50%;
	}

	.status-count {
		font-family: var(--font-mono);
		font-weight: 600;
		color: var(--text-primary);
	}

	.status-label {
		color: var(--text-muted);
		font-weight: 500;
	}

	.status-pill.working .status-indicator {
		background: var(--status-working);
		box-shadow: 0 0 8px var(--status-working-glow);
	}

	.status-pill.permission .status-indicator {
		background: var(--status-permission);
		box-shadow: 0 0 8px var(--status-permission-glow);
		animation: status-pulse 1.5s ease-in-out infinite;
	}

	.status-pill.input .status-indicator {
		background: var(--status-input);
		box-shadow: 0 0 8px var(--status-input-glow);
	}

	.status-pill.connecting .status-indicator {
		background: var(--status-connecting);
	}

	.divider {
		width: 1px;
		height: 24px;
		background: var(--border-default);
	}

	.live-indicator {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
	}

	.live-dot {
		position: relative;
		width: 8px;
		height: 8px;
		border-radius: 50%;
		background: var(--accent-green);
	}

	.live-dot::before {
		content: '';
		position: absolute;
		inset: 0;
		border-radius: 50%;
		background: var(--accent-green);
		animation: ring-pulse 2s ease-out infinite;
	}

	.live-text {
		font-family: var(--font-mono);
		font-size: 11px;
		font-weight: 600;
		color: var(--accent-green);
		letter-spacing: 0.1em;
	}
</style>
