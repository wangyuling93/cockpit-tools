import {
  WorkbuddyAccount,
  getWorkbuddyAccountDisplayEmail,
  getWorkbuddyPlanBadge,
  getWorkbuddyUsage,
} from '../types/workbuddy';
import * as workbuddyService from '../services/workbuddyService';
import { createProviderAccountStore } from './createProviderAccountStore';

const WORKBUDDY_ACCOUNTS_CACHE_KEY = 'agtools.workbuddy.accounts.cache';

export const useWorkbuddyAccountStore = createProviderAccountStore<WorkbuddyAccount>(
  WORKBUDDY_ACCOUNTS_CACHE_KEY,
  {
    listAccounts: workbuddyService.listWorkbuddyAccounts,
    deleteAccount: workbuddyService.deleteWorkbuddyAccount,
    deleteAccounts: workbuddyService.deleteWorkbuddyAccounts,
    injectAccount: workbuddyService.injectWorkbuddyToVSCode,
    refreshToken: workbuddyService.refreshWorkbuddyToken,
    refreshAllTokens: workbuddyService.refreshAllWorkbuddyTokens,
    importFromJson: workbuddyService.importWorkbuddyFromJson,
    exportAccounts: workbuddyService.exportWorkbuddyAccounts,
    updateAccountTags: workbuddyService.updateWorkbuddyAccountTags,
  },
  {
    getDisplayEmail: getWorkbuddyAccountDisplayEmail,
    getPlanBadge: getWorkbuddyPlanBadge,
    getUsage: getWorkbuddyUsage,
  },
);
