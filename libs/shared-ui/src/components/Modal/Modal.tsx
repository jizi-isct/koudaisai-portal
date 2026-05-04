"use client";

import * as React from "react";
import {Modal as AntdModal} from "antd";

type Props = {
  isOpen: boolean,
  setOpen: (isOpen: boolean) => void
  children: React.ReactNode,
}

export function Modal({isOpen, setOpen, children}: Props) {
  function closeModal() {
    setOpen(false);
  }

  return (
    <AntdModal
      open={isOpen}
      onCancel={closeModal}
      footer={null}
      centered
      width="90vw"
      styles={{
        body: {
          flex: 1,
          overflow: 'hidden',
          display: 'flex',
          flexDirection: 'column',
        },
        mask: {
          backdropFilter: 'blur(8px)',
        },
      }}
      style={{maxWidth: '90vw'}}
    >
      <div className="h-[calc(100%-5em)] max-h-[calc(100%-5em)] overflow-y-auto">
        {children}
      </div>
    </AntdModal>
  )
}