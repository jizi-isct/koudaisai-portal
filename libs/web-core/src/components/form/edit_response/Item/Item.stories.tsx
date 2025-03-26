import type {Meta, StoryObj} from '@storybook/react';
import Item from './Item';
import {Item as Item_} from '../../../../lib/types';

const meta: Meta<typeof Item> = {
  title: 'Form/EditResponse/Item',
  component: Item,
  parameters: {
    layout: 'fullscreen',
  },
};

export default meta;

type Story = StoryObj<typeof Item>;

export const TextQuestion: Story = {
  args: {
    item: {
      item_id: '1',
      title: 'Your Name',
      description: 'Please enter your full name',
      item_text: undefined,
      item_question: {
        question: {
          required: true,
          question_text: {
            paragraph: false
          },
          question_radio_button: undefined,
          question_check_box: undefined
        }
      },
      item_page_break: undefined
    } as Item_,
    setValue: (value) => console.log('Value updated:', value)
  },
};

export const ParagraphQuestion: Story = {
  args: {
    item: {
      item_id: '2',
      title: 'Additional Comments',
      description: 'Please provide any additional information',
      item_text: undefined,
      item_question: {
        question: {
          required: false,
          question_text: {
            paragraph: true
          },
          question_radio_button: undefined,
          question_check_box: undefined
        }
      },
      item_page_break: undefined
    } as Item_,
    setValue: (value) => console.log('Value updated:', value)
  },
};
