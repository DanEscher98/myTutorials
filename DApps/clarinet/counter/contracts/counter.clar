
;; title: counter
;; version:
;; summary:
;; description:

;; traits
;;

;; token definitions
;;

;; constants
;;

;; data vars
;;

;; data maps
;;

;; public functions
;;

;; read only functions
;;

;; private functions
;;

(define-data-var count uint u0)
(define-data-var contract-owner principal tx-sender)
(define-data-var const uint u10)

(define-read-only (get-count)
                 (var-get count))

(define-public (increment)
               (begin
                 (print u"incrementing count")
                 (ok (var-set count (+ (var-get count) u1)))))

(define-read-only (say-hi)
                  (ok "Hello World"))

;; A read-only function that returns an input number
(define-read-only (echo-number (val int))
                  (ok val))

;; A public function that conditionally returns an ok or an error
(define-public (check-it (flag bool))
               (if flag (ok 1) (err u100)))

(define-public (mul (val uint))
               (ok (var-set count (* (var-get count) val))))

(define-read-only (echo-double)
                  (ok (* (var-get count) u2)))
