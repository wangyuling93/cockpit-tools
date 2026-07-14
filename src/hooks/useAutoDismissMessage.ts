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

/**
 * String notice state (e.g. API Service success banners) that auto-clears after a delay.
 * Empty string is treated as “no notice”.
 */
export function useAutoDismissText(
  delayMs: number = DEFAULT_DISMISS_MS,
): [string, Dispatch<SetStateAction<string>>] {
  const [text, setText] = useState('');

  useEffect(() => {
    if (!text) return;
    const timer = window.setTimeout(() => setText(''), delayMs);
    return () => window.clearTimeout(timer);
  }, [text, delayMs]);

  return [text, setText];
}
