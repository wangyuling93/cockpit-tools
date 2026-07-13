import { useEffect, useState, type Dispatch, type SetStateAction } from 'react';

const DEFAULT_DISMISS_MS = 3000;

type MessageLike = {
  text: string;
  tone?: string;
};

/**
 * Action-message / toast state that auto-dismisses after a delay (default 3s).
 * Error-tone messages stay until the user dismisses them manually.
 */
export function useAutoDismissMessage<T extends MessageLike = MessageLike>(
  delayMs: number = DEFAULT_DISMISS_MS,
): [T | null, Dispatch<SetStateAction<T | null>>] {
  const [message, setMessage] = useState<T | null>(null);

  useEffect(() => {
    if (!message || message.tone === 'error') return;
    const timer = window.setTimeout(() => setMessage(null), delayMs);
    return () => window.clearTimeout(timer);
  }, [message, delayMs]);

  return [message, setMessage];
}
