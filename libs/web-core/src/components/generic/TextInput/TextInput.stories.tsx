import type {Meta, StoryObj} from '@storybook/react';
import TextInput from './TextInput';

const meta: Meta<typeof TextInput> = {
  title: 'Generic/TextInput',
  component: TextInput,
};

export default meta;

type Story = StoryObj<typeof TextInput>;

export const Default: Story = {
  args: {
    placeholder: 'Enter text...',
    paragraph: false,
    setValue: (value: string) => console.log(value),
  },
};

export const Paragraph: Story = {
  args: {
    placeholder: 'Enter long text...',
    paragraph: true,
    setValue: (value: string) => console.log(value),
  },
};

export const WithWidth: Story = {
  args: {
    placeholder: 'Width constrained',
    width: 300,
    setValue: (value: string) => console.log(value),
  },
};

export const WithValue: Story = {
  args: {
    value: 'Initial value',
    setValue: (value: string) => console.log(value),
  },
};
