import type {Meta, StoryObj} from '@storybook/react';
import EditForm from './EditForm';
import {Form} from '../../../../lib/types';

const meta: Meta<typeof EditForm> = {
  title: 'Form/EditForm/Main',
  component: EditForm,
  parameters: {
    layout: 'fullscreen',
  },
};

export default meta;

type Story = StoryObj<typeof EditForm>;

export const Default: Story = {
  args: {
    form: {
      form_id: '1',
      created_at: '2025-03-25T12:00:00Z',
      updated_at: '2025-03-25T12:00:00Z',
      info: {
        title: 'Event Registration Form',
        document_title: '2025 Annual Event Registration',
        description: 'Please fill out all required fields',
        deadline: undefined
      },
      access_control: {
        roles: ['none']
      },
      items: []
    } as Form,
    setForm: (form) => console.log('Form updated:', form)
  },
};
