"use client";

import {default as ReactModal} from "react-modal"
import * as React from "react";
import {Button} from "../Button";
import styles from "./Modal.module.css";
import {useEffect} from "react";


type Props = {
  isOpen: boolean,
  setOpen: (isOpen: boolean) => void
  children: React.ReactNode,
}

export function Modal({isOpen, setOpen, children}: Props) {
  function closeModal() {
    setOpen(false);
  }

  const [appElement, setAppElement] = React.useState<HTMLElement | null>(null);

  useEffect(() => {
    setAppElement(document.getElementById("app"));
  })


  return (
    <ReactModal
      isOpen={isOpen}
      onRequestClose={closeModal}
      className={styles.modalWindow}
      overlayClassName={styles.modalOverlay}
      appElement={appElement!}
    >
      <div className={styles.children}>
        {children}
      </div>
      <div className={styles.closeButton}>
        <Button text={"閉じる"} onClick={closeModal} />
      </div>
    </ReactModal>
  )
}