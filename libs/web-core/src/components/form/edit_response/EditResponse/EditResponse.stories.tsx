import type {Meta, StoryObj} from '@storybook/react';
import EditResponse from './EditResponse';
import {Form} from '../../../../lib/types';

// Import FormResponse type from the component's Props type
type Props = React.ComponentProps<typeof EditResponse>;
type FormResponse = Props['formResponse'];

const meta: Meta<typeof EditResponse> = {
  title: 'Form/EditResponse/Main',
  component: EditResponse,
  parameters: {
    layout: 'fullscreen',
  },
};

export default meta;

type Story = StoryObj<typeof EditResponse>;

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
    formResponse: {
      response_id: '123',
      form_id: '1',
      created_at: '2025-03-25T14:00:00Z',
      updated_at: '2025-03-25T14:30:00Z',
      answers: {}
    } as any, // Using any temporarily to bypass type checking
    setFormResponse: (formResponse) => console.log('Response updated:', formResponse)
  },
};
