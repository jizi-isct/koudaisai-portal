import type {Meta, StoryObj} from '@storybook/react';
import Item from './Item';
import {Item as Item_} from '../../../../lib/types';

const meta: Meta<typeof Item> = {
  title: 'Form/EditForm/Item',
  component: Item,
  parameters: {
    layout: 'fullscreen',
  },
};

export default meta;

type Story = StoryObj<typeof Item>;

export const TextItem: Story = {
  args: {
    item: {
      item_id: '1',
      title: 'Information Section',
      description: 'This section provides important event information',
      item_text: {},
      item_question: undefined,
      item_page_break: undefined
    } as Item_,
    setItem: (item) => console.log('Item updated:', item),
    moveUp: () => console.log('Move up clicked'),
    moveDown: () => console.log('Move down clicked'),
    delete_: () => console.log('Delete clicked')
  },
};

export const QuestionTextItem: Story = {
  args: {
    item: {
      item_id: '2',
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
    setItem: (item) => console.log('Item updated:', item),
    moveUp: () => console.log('Move up clicked'),
    moveDown: () => console.log('Move down clicked'),
    delete_: () => console.log('Delete clicked')
  },
};
