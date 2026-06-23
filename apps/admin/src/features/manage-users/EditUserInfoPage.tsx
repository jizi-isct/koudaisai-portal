import { $api } from '@/features/api/api';
import { LoadingScreen, Heading1 } from '@koudaisai/shared-ui';
import { useState, useEffect } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { formatDate } from '@koudaisai/shared-utils';
import {
  Descriptions,
  Tag,
  Flex,
  Button,
  Result,
  type DescriptionsProps,
} from 'antd';

export function EditUserInfoPage(){
    return(
        <Heading1 emoji="">実装中</Heading1>
    )
}