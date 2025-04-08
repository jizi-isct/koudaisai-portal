import type {Meta, StoryObj} from '@storybook/react';
import SaveStatus from './SaveStatus';

const meta: Meta<typeof SaveStatus> = {
  title: 'Form/EditForm/SaveStatus',
  component: SaveStatus,
  parameters: {
    layout: 'fullscreen',
  },
};

export default meta;

type Story = StoryObj<typeof SaveStatus>;

export const Saved: Story = {
  args: {
    status: 'saved'
  },
};

export const Unsaved: Story = {
  args: {
    status: 'unsaved'
  },
};

export const Saving: Story = {
  args: {
    status: 'saving'
  },
};
