import type {Meta, StoryObj} from '@storybook/react';
import SelectFormItemType from './SelectFormItemType';
import {FormItemType} from '../../../../lib/types';

const meta: Meta<typeof SelectFormItemType> = {
  title: 'Form/EditForm/SelectFormItemType',
  component: SelectFormItemType,
  parameters: {
    layout: 'centered',
  },
};

export default meta;

type Story = StoryObj<typeof SelectFormItemType>;

export const Text: Story = {
  args: {
    value: 'text' as FormItemType,
    onChange: (value) => console.log('Selected:', value)
  },
};

export const PageBreak: Story = {
  args: {
    value: 'page_break' as FormItemType,
    onChange: (value) => console.log('Selected:', value)
  },
};

export const QuestionText: Story = {
  args: {
    value: 'question_text' as FormItemType,
    onChange: (value) => console.log('Selected:', value)
  },
};

export const QuestionRadioButton: Story = {
  args: {
    value: 'question_radio_button' as FormItemType,
    onChange: (value) => console.log('Selected:', value)
  },
};

export const QuestionCheckBox: Story = {
  args: {
    value: 'question_check_box' as FormItemType,
    onChange: (value) => console.log('Selected:', value)
  },
};
