import type {Meta, StoryObj} from '@storybook/react';
import {Button} from './Button';

const meta: Meta<typeof Button> = {
  title: 'Generic/Button',
  component: Button,
};

export default meta;

type Story = StoryObj<typeof Button>;

export const Default: Story = {
  args: {
    text: 'Click me',
    onClick: () => alert('Button clicked!'),
  },
};

export const CustomColor: Story = {
  args: {
    text: 'Custom Color',
    color: '#FF5733',
    onClick: () => alert('Custom color button clicked!'),
  },
};

export const ClickedState: Story = {
  args: {
    text: 'Clicked State',
    isClicked: true,
    onClick: () => alert('Clicked state button clicked!'),
  },
};
