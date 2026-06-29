import type { Page } from '../types/navigation';

export const APIKEY_FUN_PREFILL_EVENT = 'app:apikey-fun-prefill';

export type ApiKeyFunPrefillTarget = 'codex' | 'claude_desktop' | 'claude_cli';

export interface ApiKeyFunPrefillPayload {
  target: ApiKeyFunPrefillTarget;
  apiKey: string;
  apiKeyName?: string | null;
  providerName?: string | null;
  baseUrl?: string | null;
  sourceTag?: string | null;
  modelCatalog?: string[] | null;
}

interface ApiKeyFunPrefillEnvelope {
  createdAt: number;
  payload: ApiKeyFunPrefillPayload;
}

type ApiKeyFunPrefillWindow = Window & {
  __agtoolsApiKeyFunPrefill?: ApiKeyFunPrefillEnvelope | null;
};

const APIKEY_FUN_PREFILL_STORAGE_KEY = 'agtools.apikeyFun.prefill.pending.v1';
const APIKEY_FUN_PREFILL_TTL_MS = 2 * 60 * 1000;

let pendingPrefill: ApiKeyFunPrefillPayload | null = null;

export function getApiKeyFunPrefillPage(target: ApiKeyFunPrefillTarget): Page {
  if (target === 'codex') return 'codex';
  return target === 'claude_desktop' ? 'claude' : 'claude-cli';
}

function isApiKeyFunPrefillTarget(value: unknown): value is ApiKeyFunPrefillTarget {
  return value === 'codex' || value === 'claude_desktop' || value === 'claude_cli';
}

function normalizeStringArray(value: unknown): string[] | null | undefined {
  if (value == null) return value as null | undefined;
  if (!Array.isArray(value)) return null;
  const values = value.filter((item): item is string => typeof item === 'string');
  return values.length === value.length ? values : null;
}

function normalizeNullableString(value: unknown): string | null | undefined {
  if (value == null) return value as null | undefined;
  return typeof value === 'string' ? value : null;
}

function normalizeApiKeyFunPrefillPayload(value: unknown): ApiKeyFunPrefillPayload | null {
  if (!value || typeof value !== 'object') return null;
  const input = value as Partial<Record<keyof ApiKeyFunPrefillPayload, unknown>>;
  if (!isApiKeyFunPrefillTarget(input.target) || typeof input.apiKey !== 'string') {
    return null;
  }

  const apiKeyName = normalizeNullableString(input.apiKeyName);
  const providerName = normalizeNullableString(input.providerName);
  const baseUrl = normalizeNullableString(input.baseUrl);
  const sourceTag = normalizeNullableString(input.sourceTag);
  const modelCatalog = normalizeStringArray(input.modelCatalog);
  if (
    apiKeyName === null && input.apiKeyName != null ||
    providerName === null && input.providerName != null ||
    baseUrl === null && input.baseUrl != null ||
    sourceTag === null && input.sourceTag != null ||
    modelCatalog === null && input.modelCatalog != null
  ) {
    return null;
  }

  return {
    target: input.target,
    apiKey: input.apiKey,
    apiKeyName,
    providerName,
    baseUrl,
    sourceTag,
    modelCatalog,
  };
}

function normalizeApiKeyFunPrefillEnvelope(value: unknown): ApiKeyFunPrefillEnvelope | null {
  if (!value || typeof value !== 'object') return null;
  const input = value as { createdAt?: unknown; payload?: unknown };
  if (typeof input.createdAt !== 'number' || !Number.isFinite(input.createdAt)) {
    return null;
  }
  const payload = normalizeApiKeyFunPrefillPayload(input.payload);
  if (!payload) return null;
  if (Date.now() - input.createdAt > APIKEY_FUN_PREFILL_TTL_MS) {
    return null;
  }
  return { createdAt: input.createdAt, payload };
}

function getPrefillWindow(): ApiKeyFunPrefillWindow | null {
  return typeof window === 'undefined' ? null : window as ApiKeyFunPrefillWindow;
}

function writeSharedPendingPrefill(payload: ApiKeyFunPrefillPayload): void {
  const envelope: ApiKeyFunPrefillEnvelope = {
    createdAt: Date.now(),
    payload,
  };
  const targetWindow = getPrefillWindow();
  if (targetWindow) {
    targetWindow.__agtoolsApiKeyFunPrefill = envelope;
  }
  try {
    targetWindow?.sessionStorage.setItem(APIKEY_FUN_PREFILL_STORAGE_KEY, JSON.stringify(envelope));
  } catch {
    // sessionStorage can be unavailable in restricted WebViews; the window field is enough then.
  }
}

function readSharedPendingPrefill(): ApiKeyFunPrefillPayload | null {
  const targetWindow = getPrefillWindow();
  const windowEnvelope = normalizeApiKeyFunPrefillEnvelope(
    targetWindow?.__agtoolsApiKeyFunPrefill,
  );
  if (windowEnvelope) {
    return windowEnvelope.payload;
  }

  try {
    const raw = targetWindow?.sessionStorage.getItem(APIKEY_FUN_PREFILL_STORAGE_KEY);
    if (!raw) return null;
    const envelope = normalizeApiKeyFunPrefillEnvelope(JSON.parse(raw));
    if (envelope) {
      return envelope.payload;
    }
    targetWindow?.sessionStorage.removeItem(APIKEY_FUN_PREFILL_STORAGE_KEY);
  } catch {
    return null;
  }
  return null;
}

function clearSharedPendingPrefill(): void {
  const targetWindow = getPrefillWindow();
  if (targetWindow) {
    targetWindow.__agtoolsApiKeyFunPrefill = null;
  }
  try {
    targetWindow?.sessionStorage.removeItem(APIKEY_FUN_PREFILL_STORAGE_KEY);
  } catch {
    // Ignore storage cleanup errors; stale entries are TTL guarded.
  }
}

export function dispatchApiKeyFunPrefillEvent(payload: ApiKeyFunPrefillPayload): void {
  pendingPrefill = payload;
  writeSharedPendingPrefill(payload);
  window.dispatchEvent(
    new CustomEvent<ApiKeyFunPrefillPayload>(APIKEY_FUN_PREFILL_EVENT, {
      detail: payload,
    }),
  );
}

export function consumeApiKeyFunPrefill(
  target: ApiKeyFunPrefillTarget,
  event?: Event,
): ApiKeyFunPrefillPayload | null {
  const eventPayload = normalizeApiKeyFunPrefillPayload(
    event instanceof CustomEvent ? event.detail : null,
  );
  if (eventPayload?.target === target) {
    pendingPrefill = null;
    clearSharedPendingPrefill();
    return eventPayload;
  }

  if (pendingPrefill?.target === target) {
    const payload = pendingPrefill;
    pendingPrefill = null;
    clearSharedPendingPrefill();
    return payload;
  }

  const sharedPayload = readSharedPendingPrefill();
  if (sharedPayload?.target === target) {
    pendingPrefill = null;
    clearSharedPendingPrefill();
    return sharedPayload;
  }

  return null;
}
