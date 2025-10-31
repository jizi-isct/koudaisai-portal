"use client";

import * as React from "react";
import {Button} from "../Button";
import styles from "./Modal.module.css";
import dynamic from "next/dynamic";

const ReactModal = dynamic(
  () => import("react-modal"),
  {ssr: false}
)

type Props = {
  isOpen: boolean,
  setOpen: (isOpen: boolean) => void
  children: React.ReactNode,
}

export function Modal({isOpen, setOpen, children}: Props) {
  function closeModal() {
    setOpen(false);
  }

  // Lock background scroll when modal is open (includes iOS Safari handling)
  React.useEffect(() => {
    if (!isOpen) return;

    const body = document.body;
    const docEl = document.documentElement;

    const prevOverflow = body.style.overflow;
    const prevPaddingRight = body.style.paddingRight;
    const prevPosition = body.style.position;
    const prevTop = body.style.top;
    const prevWidth = body.style.width;

    const scrollY = window.scrollY || window.pageYOffset;
    const scrollbarWidth = window.innerWidth - docEl.clientWidth;

    // Basic lock
    body.style.overflow = 'hidden';
    if (scrollbarWidth > 0) {
      body.style.paddingRight = `${scrollbarWidth}px`;
    }

    // iOS safari fix: use position: fixed to truly lock scrolling
    const isIOS = /iP(ad|hone|od)/.test(navigator.platform) ||
      (navigator.userAgent.includes('Mac') && 'ontouchend' in document);
    if (isIOS) {
      body.style.position = 'fixed';
      body.style.top = `-${scrollY}px`;
      body.style.width = '100%';
    }

    return () => {
      body.style.overflow = prevOverflow;
      body.style.paddingRight = prevPaddingRight;
      if (isIOS) {
        body.style.position = prevPosition;
        body.style.top = prevTop;
        body.style.width = prevWidth;
        // restore original scroll position
        window.scrollTo(0, scrollY);
      }
    };
  }, [isOpen]);

  const appElement = document.getElementById("app")


  return (
    <ReactModal
      isOpen={isOpen}
      onRequestClose={closeModal}
      className={styles.modalWindow}
      overlayClassName={styles.modalOverlay}
      appElement={appElement ?? undefined}
      ariaHideApp={appElement !== null}
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