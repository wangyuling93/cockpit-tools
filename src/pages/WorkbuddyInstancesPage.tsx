import { useTranslation } from 'react-i18next';
import { PlatformInstancesContent } from '../components/platform/PlatformInstancesContent';
import { useWorkbuddyInstanceStore } from '../stores/useWorkbuddyInstanceStore';
import { useWorkbuddyAccountStore } from '../stores/useWorkbuddyAccountStore';
import type { WorkbuddyAccount } from '../types/workbuddy';
import {
  getWorkbuddyAccountDisplayEmail,
  getWorkbuddyPlanBadge,
  getWorkbuddyUsage,
} from '../types/workbuddy';
import { usePlatformRuntimeSupport } from '../hooks/usePlatformRuntimeSupport';

interface WorkbuddyInstancesContentProps {
  accountsForSelect?: WorkbuddyAccount[];
}

export function WorkbuddyInstancesContent({
  accountsForSelect,
}: WorkbuddyInstancesContentProps = {}) {
  const { t, i18n } = useTranslation();
  const locale = i18n.language || 'zh-CN';
  const instanceStore = useWorkbuddyInstanceStore();
  const { accounts: storeAccounts, fetchAccounts } = useWorkbuddyAccountStore();
  const sourceAccounts = accountsForSelect ?? storeAccounts;
  const isSupportedPlatform = usePlatformRuntimeSupport('desktop');

  const renderWorkbuddyQuotaPreview = (account: WorkbuddyAccount) => {
    const usage = getWorkbuddyUsage(account);
    if (!usage.dosageNotifyCode) {
      return <span className="account-quota-empty">{t('instances.quota.empty', '暂无配额缓存')}</span>;
    }
    if (usage.isNormal) {
      return (
        <div className="account-quota-preview">
          <span className="account-quota-item">
            <span className="quota-dot high" />
            <span className="quota-text high">{t('workbuddy.usageNormal', '正常')}</span>
          </span>
        </div>
      );
    }

    const text = locale.startsWith('zh')
      ? (usage.dosageNotifyZh || usage.dosageNotifyCode)
      : (usage.dosageNotifyEn || usage.dosageNotifyCode);

    return (
      <div className="account-quota-preview">
        <span className="account-quota-item">
          <span className="quota-dot critical" />
          <span className="quota-text critical">{text}</span>
        </span>
      </div>
    );
  };

  return (
    <PlatformInstancesContent<WorkbuddyAccount>
      instanceStore={instanceStore}
      accounts={sourceAccounts}
      fetchAccounts={fetchAccounts}
      renderAccountQuotaPreview={renderWorkbuddyQuotaPreview}
      renderAccountBadge={(account) => {
        const planBadge = getWorkbuddyPlanBadge(account);
        const normalizedClass = planBadge.toLowerCase();
        return <span className={`instance-plan-badge ${normalizedClass}`}>{planBadge}</span>;
      }}
      getAccountSearchText={(account) => `${getWorkbuddyAccountDisplayEmail(account)} ${getWorkbuddyPlanBadge(account)}`}
      appType="workbuddy"
      isSupported={isSupportedPlatform}
      unsupportedTitleKey="common.shared.instances.unsupported.title"
      unsupportedTitleDefault="暂不支持当前系统"
      unsupportedDescKey="workbuddy.instances.unsupported.descPlatform"
      unsupportedDescDefault="WorkBuddy 多开实例仅支持 macOS、Windows 和 Linux。"
    />
  );
}
