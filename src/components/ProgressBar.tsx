interface ProgressBarProps {
  progress: number; // 0-100
  indeterminate?: boolean;
  label?: string;
}

export function ProgressBar({ progress, indeterminate, label = "Progress" }: ProgressBarProps) {
  const clamped = Math.min(100, Math.max(0, progress));

  if (indeterminate) {
    return (
      <div
        className="w-full overflow-hidden rounded-full"
        role="progressbar"
        aria-label={label}
        aria-valuemin={0}
        aria-valuemax={100}
        style={{
          height: "4px",
          backgroundColor: "var(--color-surface-elevated)",
        }}
      >
        <div
          className="h-full rounded-full"
          style={{
            background: "linear-gradient(90deg, var(--color-primary), var(--color-accent-purple), var(--color-primary))",
            backgroundSize: "200% 100%",
            animation: "shimmer 1.5s linear infinite",
            width: "40%",
          }}
        />
        <style>{`@keyframes shimmer { 0% { background-position: 200% 0; transform: translateX(-100%); } 100% { background-position: -200% 0; transform: translateX(250%); } }`}</style>
      </div>
    );
  }

  return (
    <div
      className="w-full overflow-hidden rounded-full"
      role="progressbar"
      aria-label={label}
      aria-valuenow={clamped}
      aria-valuemin={0}
      aria-valuemax={100}
      style={{
        height: "4px",
        backgroundColor: "var(--color-surface-elevated)",
      }}
    >
      <div
        className="h-full rounded-full transition-all duration-300 ease-snap-out"
        style={{
          width: `${clamped}%`,
          backgroundColor: "var(--color-primary)",
        }}
      />
    </div>
  );
}
