import type {Meta, StoryObj} from '@storybook/react';
import Info from './Info';
import {Info as Info_} from '../../../../lib/types';

const meta: Meta<typeof Info> = {
  title: 'Form/EditResponse/Info',
  component: Info,
  parameters: {
    layout: 'fullscreen',
  },
};

export default meta;

type Story = StoryObj<typeof Info>;

export const Default: Story = {
  args: {
    info: {
      title: 'Event Registration Form',
      document_title: '2025 Annual Event Registration',
      description: 'Please fill out all required fields. Registration deadline is April 10th.',
      deadline: '2025-04-10T23:59:59Z'
    } as Info_
  },
};

export const NoDeadline: Story = {
  args: {
    info: {
      title: 'Feedback Survey',
      document_title: 'Customer Feedback Survey',
      description: 'Help us improve our services by providing your feedback.',
      deadline: undefined
    } as Info_
  },
};
