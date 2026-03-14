export interface WorkbuddyAccount {
  id: string;
  email: string;
  uid?: string | null;
  nickname?: string | null;
  enterprise_id?: string | null;
  enterprise_name?: string | null;
  tags?: string[] | null;

  access_token: string;
  refresh_token?: string | null;
  token_type?: string | null;
  expires_at?: number | null;
  domain?: string | null;

  plan_type?: string;
  dosage_notify_code?: string;
  dosage_notify_zh?: string;
  dosage_notify_en?: string;
  payment_type?: string;

  quota_raw?: unknown;
  auth_raw?: unknown;
  profile_raw?: unknown;
  usage_raw?: unknown;

  status?: string | null;
  status_reason?: string | null;
  quota_query_last_error?: string | null;
  quota_query_last_error_at?: number | null;

  created_at: number;
  last_used: number;
}

export type WorkbuddyPlanBadge = 'FREE' | 'PRO' | 'TRIAL' | 'ENTERPRISE' | 'UNKNOWN';

export const WORKBUDDY_PACKAGE_CODE = {
  free: 'TCACA_code_001_PqouKr6QWV',
  proMon: 'TCACA_code_002_AkiJS3ZHF5',
  proYear: 'TCACA_code_003_FAnt7lcmRT',
  gift: 'TCACA_code_006_DbXS0lrypC',
  activity: 'TCACA_code_007_nzdH5h4Nl0',
  freeMon: 'TCACA_code_008_cfWoLwvjU4',
  extra: 'TCACA_code_009_0XmEQc2xOf',
} as const;

export const WORKBUDDY_RESOURCE_STATUS = {
  valid: 0,
  refund: 1,
  expired: 2,
  usedUp: 3,
} as const;

const WORKBUDDY_ENTERPRISE_ACCOUNT_TYPES = ['ultimate', 'exclusive', 'premise'];

export interface WorkbuddyPlanDetail {
  type: 'pro' | 'free';
  isPro: boolean;
  isTrial: boolean;
  badge: string;
  packageCode: string | null;
}

export function getWorkbuddyAccountDisplayEmail(account: WorkbuddyAccount): string {
  return account.email || account.nickname || account.uid || account.id;
}

export function getWorkbuddyAccountDisplayName(account: WorkbuddyAccount): string {
  return account.nickname || account.email || account.uid || account.id;
}

export function getWorkbuddyPlanDetail(account: WorkbuddyAccount): WorkbuddyPlanDetail {
  // 1. 先检查企业账号类型
  const profile = asRecord(account.profile_raw);
  const accountType = typeof profile?.type === 'string' ? profile.type.toLowerCase() : '';
  if (WORKBUDDY_ENTERPRISE_ACCOUNT_TYPES.includes(accountType)) {
    return { type: 'pro', isPro: true, isTrial: false, badge: 'ENTERPRISE', packageCode: null };
  }

  // 2. 提取资源账号
  const all = extractResourceAccounts(account);
  const active = all.filter((a) => {
    const s = typeof a.Status === 'number' ? a.Status : -1;
    return s === WORKBUDDY_RESOURCE_STATUS.valid || s === WORKBUDDY_RESOURCE_STATUS.usedUp;
  });

  // 3. 检查专业版套餐
  const proPkg = active.find((a) => {
    const c = typeof a.PackageCode === 'string' ? a.PackageCode : '';
    return c === WORKBUDDY_PACKAGE_CODE.proYear || c === WORKBUDDY_PACKAGE_CODE.proMon;
  });

  const hasGift = active.some((a) => {
    const c = typeof a.PackageCode === 'string' ? a.PackageCode : '';
    return c === WORKBUDDY_PACKAGE_CODE.gift;
  });

  if (proPkg) {
    const code = typeof proPkg.PackageCode === 'string' ? proPkg.PackageCode : null;
    return { type: 'pro', isPro: true, isTrial: hasGift, badge: 'PRO', packageCode: code };
  }

  if (hasGift) {
    return { type: 'free', isPro: false, isTrial: true, badge: 'TRIAL', packageCode: WORKBUDDY_PACKAGE_CODE.gift };
  }

  // 4. 回退逻辑
  if (all.length === 0) {
    return planBadgeFallback(account);
  }

  return { type: 'free', isPro: false, isTrial: false, badge: 'FREE', packageCode: null };
}

function planBadgeFallback(account: WorkbuddyAccount): WorkbuddyPlanDetail {
  const payment = account.payment_type?.toLowerCase() || '';
  const plan = account.plan_type?.toLowerCase() || '';
  const source = payment || plan;

  if (source.includes('enterprise'))
    return { type: 'pro', isPro: true, isTrial: false, badge: 'ENTERPRISE', packageCode: null };
  if (source.includes('trial'))
    return { type: 'free', isPro: false, isTrial: true, badge: 'TRIAL', packageCode: null };
  if (source.includes('pro'))
    return { type: 'pro', isPro: true, isTrial: false, badge: 'PRO', packageCode: null };
  if (source.includes('free'))
    return { type: 'free', isPro: false, isTrial: false, badge: 'FREE', packageCode: null };
  if (source) {
    const raw = (account.payment_type || account.plan_type || 'UNKNOWN').toUpperCase();
    return { type: 'free', isPro: false, isTrial: false, badge: raw, packageCode: null };
  }
  return { type: 'free', isPro: false, isTrial: false, badge: 'UNKNOWN', packageCode: null };
}

export function getWorkbuddyPlanBadge(account: WorkbuddyAccount): string {
  return getWorkbuddyPlanDetail(account).badge;
}

export interface WorkbuddyUsage {
  dosageNotifyCode?: string;
  dosageNotifyZh?: string;
  dosageNotifyEn?: string;
  paymentType?: string;
  isNormal: boolean;
  inlineSuggestionsUsedPercent: number | null;
  chatMessagesUsedPercent: number | null;
  allowanceResetAt?: number | null;
}

export function getWorkbuddyUsage(account: WorkbuddyAccount): WorkbuddyUsage {
  const code = account.dosage_notify_code || '';
  return {
    dosageNotifyCode: code,
    dosageNotifyZh: account.dosage_notify_zh || undefined,
    dosageNotifyEn: account.dosage_notify_en || undefined,
    paymentType: account.payment_type || undefined,
    isNormal: !code || code === '0' || code === 'USAGE_NORMAL',
    inlineSuggestionsUsedPercent: null,
    chatMessagesUsedPercent: null,
    allowanceResetAt: null,
  };
}

export interface WorkbuddyOfficialQuotaResource {
  packageCode: string | null;
  packageName: string | null;
  cycleStartTime: string | null;
  cycleEndTime: string | null;
  deductionEndTime: number | null;
  expiredTime: string | null;
  total: number;
  remain: number;
  used: number;
  usedPercent: number;
  remainPercent: number | null;
  refreshAt: number | null;
  expireAt: number | null;
  isBasePackage: boolean;
}

export interface WorkbuddyOfficialQuotaModel {
  resources: WorkbuddyOfficialQuotaResource[];
  extra: WorkbuddyOfficialQuotaResource;
  updatedAt: number | null;
}

function asRecord(value: unknown): Record<string, unknown> | null {
  return value && typeof value === 'object' ? (value as Record<string, unknown>) : null;
}

function parseNumeric(value: unknown): number | null {
  if (typeof value === 'number' && Number.isFinite(value)) return value;
  if (typeof value === 'string' && value.trim()) {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : null;
  }
  return null;
}

function isExtraPackage(a: Record<string, unknown>): boolean {
  return typeof a.PackageCode === 'string' && a.PackageCode === WORKBUDDY_PACKAGE_CODE.extra;
}

function isActiveResource(a: Record<string, unknown>): boolean {
  const s = typeof a.Status === 'number' ? a.Status : -1;
  return s === WORKBUDDY_RESOURCE_STATUS.valid || s === WORKBUDDY_RESOURCE_STATUS.usedUp;
}

function parseDateTimeToEpoch(value: unknown): number | null {
  if (typeof value !== 'string') return null;
  const text = value.trim();
  if (!text) return null;
  const isoText = text.includes('T') ? text : text.replace(' ', 'T');
  const parsed = Date.parse(isoText);
  return Number.isFinite(parsed) ? parsed : null;
}

function parseCycleTotal(a: Record<string, unknown>): number {
  return (
    parseNumeric(a.CycleCapacitySizePrecise) ??
    parseNumeric(a.CycleCapacitySize) ??
    parseNumeric(a.CapacitySizePrecise) ??
    parseNumeric(a.CapacitySize) ??
    0
  );
}

function parseCycleRemain(a: Record<string, unknown>): number {
  return (
    parseNumeric(a.CycleCapacityRemainPrecise) ??
    parseNumeric(a.CycleCapacityRemain) ??
    parseNumeric(a.CapacityRemainPrecise) ??
    parseNumeric(a.CapacityRemain) ??
    0
  );
}

function toOfficialQuotaResource(raw: Record<string, unknown>): WorkbuddyOfficialQuotaResource {
  const packageCode = typeof raw.PackageCode === 'string' ? raw.PackageCode : null;
  const packageName = typeof raw.PackageName === 'string' ? raw.PackageName : null;
  const cycleStartTime = typeof raw.CycleStartTime === 'string' ? raw.CycleStartTime : null;
  const cycleEndTime = typeof raw.CycleEndTime === 'string' ? raw.CycleEndTime : null;
  const deductionEndTime = parseNumeric(raw.DeductionEndTime);
  const expiredTime = typeof raw.ExpiredTime === 'string' ? raw.ExpiredTime : null;

  const total = parseCycleTotal(raw);
  const remain = parseCycleRemain(raw);
  const used = Math.max(0, total - remain);
  const usedPercent = total > 0 ? Math.max(0, Math.min(100, (used / total) * 100)) : 0;
  const remainPercent = total > 0 ? Math.max(0, Math.min(100, (remain / total) * 100)) : null;

  const refreshAt = parseDateTimeToEpoch(cycleStartTime);
  const expireAt = parseDateTimeToEpoch(cycleEndTime);

  return {
    packageCode,
    packageName,
    cycleStartTime,
    cycleEndTime,
    deductionEndTime,
    expiredTime,
    total,
    remain,
    used,
    usedPercent,
    remainPercent,
    refreshAt,
    expireAt,
    isBasePackage: !isExtraPackage(raw),
  };
}

function extractResourceAccounts(account: WorkbuddyAccount): Array<Record<string, unknown>> {
  const usageRoot = asRecord(account.usage_raw);
  const quotaRoot = asRecord(account.quota_raw);
  const userResource = asRecord(quotaRoot?.userResource) ?? usageRoot;
  const data = asRecord(userResource?.data);
  const response = asRecord(data?.Response);
  const payload = asRecord(response?.Data);
  const list = Array.isArray(payload?.Accounts) ? (payload!.Accounts as unknown[]) : [];
  return list.filter((a): a is Record<string, unknown> => a != null && typeof a === 'object');
}

export function getWorkbuddyOfficialQuotaModel(account: WorkbuddyAccount): WorkbuddyOfficialQuotaModel {
  const all = extractResourceAccounts(account);
  const active = all.filter(isActiveResource);

  const resources = active
    .filter((a) => !isExtraPackage(a))
    .map(toOfficialQuotaResource)
    .filter((r) => r.total > 0 || r.remain > 0);

  const extra = (() => {
    const extraRaw = active.find(isExtraPackage);
    if (!extraRaw) {
      return {
        packageCode: WORKBUDDY_PACKAGE_CODE.extra,
        packageName: '加量包',
        cycleStartTime: null,
        cycleEndTime: null,
        deductionEndTime: null,
        expiredTime: null,
        total: 0,
        remain: 0,
        used: 0,
        usedPercent: 0,
        remainPercent: null,
        refreshAt: null,
        expireAt: null,
        isBasePackage: false,
      };
    }
    return {
      ...toOfficialQuotaResource(extraRaw),
      packageName: '加量包',
    };
  })();

  const updatedAt = getAccountQuotaUpdatedAtMs(account);

  return {
    resources,
    extra,
    updatedAt,
  };
}

function getAccountQuotaUpdatedAtMs(account: WorkbuddyAccount): number | null {
  const lastUsed = account.last_used;
  if (typeof lastUsed !== 'number' || !Number.isFinite(lastUsed) || lastUsed <= 0) return null;
  return Math.trunc(lastUsed * 1000);
}

/** 配额显示项 */
export interface WorkbuddyQuotaDisplayItem {
  key: string;
  label: string;
  used: number;
  total: number;
  remain: number;
  usedPercent: number;
  remainPercent: number | null;
  quotaClass: string;
  refreshAt: number | null;
}

/** 获取 WorkBuddy 配额显示项列表 */
export function getWorkbuddyQuotaDisplayItems(account: WorkbuddyAccount, _t: (key: string, defaultValue?: string) => string): WorkbuddyQuotaDisplayItem[] {
  const model = getWorkbuddyOfficialQuotaModel(account);
  const items: WorkbuddyQuotaDisplayItem[] = [];

  // 添加基础包
  for (const resource of model.resources) {
    if (resource.total <= 0 && resource.remain <= 0) continue;

    const remainPercent = resource.remainPercent ?? Math.max(0, 100 - resource.usedPercent);
    const quotaClass = remainPercent <= 10 ? 'low' : remainPercent <= 30 ? 'medium' : 'high';

    items.push({
      key: `base-${resource.packageCode || items.length}`,
      label: resource.packageName || '基础包',
      used: resource.used,
      total: resource.total,
      remain: resource.remain,
      usedPercent: resource.usedPercent,
      remainPercent: resource.remainPercent,
      quotaClass,
      refreshAt: resource.refreshAt,
    });
  }

  // 添加加量包
  if (model.extra.total > 0 || model.extra.remain > 0) {
    const remainPercent = model.extra.remainPercent ?? Math.max(0, 100 - model.extra.usedPercent);
    const quotaClass = remainPercent <= 10 ? 'low' : remainPercent <= 30 ? 'medium' : 'high';

    items.push({
      key: 'extra',
      label: '加量包',
      used: model.extra.used,
      total: model.extra.total,
      remain: model.extra.remain,
      usedPercent: model.extra.usedPercent,
      remainPercent: model.extra.remainPercent,
      quotaClass,
      refreshAt: model.extra.refreshAt,
    });
  }

  return items; // 返回所有项，由 UI 层决定如何展示
}
