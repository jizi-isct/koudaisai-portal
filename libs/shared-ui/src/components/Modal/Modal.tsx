"use client";

import * as React from "react";
import {Button} from "../Button";
import styles from "./Modal.module.css";
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
        root: {
          height: '90vh',
          border: '1px solid var(--darkblue)',
          borderRadius: '10px',
          boxShadow: '4px 4px 0 0 var(--darkblue)',
          padding: '15px',
          display: 'flex',
          flexDirection: 'column',
        },
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
      <div className={styles.children}>
        {children}
      </div>
      <div className={styles.closeButton}>
        <Button text={"閉じる"} onClick={closeModal} />
      </div>
    </AntdModal>
  )
}
