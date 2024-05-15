;;; tula-mode.el --- Major Mode for editing Tula source code -*- lexical-binding: t -*-

;; Copyright (C) 2024 Alexey Kutepov <reximkut@gmail.com>

;; Author: Alexey Kutepov <reximkut@gmail.com>
;; URL: https://github.com/tsoding/tula

;; Permission is hereby granted, free of charge, to any person
;; obtaining a copy of this software and associated documentation
;; files (the "Software"), to deal in the Software without
;; restriction, including without limitation the rights to use, copy,
;; modify, merge, publish, distribute, sublicense, and/or sell copies
;; of the Software, and to permit persons to whom the Software is
;; furnished to do so, subject to the following conditions:

;; The above copyright notice and this permission notice shall be
;; included in all copies or substantial portions of the Software.

;; THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
;; EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
;; MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
;; NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
;; BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
;; ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
;; CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
;; SOFTWARE.

;;; Commentary:
;;
;; Major Mode for editing Tula source code. It's an Esoteric Language
;; based on Turing Machine and extended with Set Theory and
;; S-expressions.

(defconst tula-mode-syntax-table
  (with-syntax-table (copy-syntax-table)
    ;; C/C++ style comments
	(modify-syntax-entry ?/ ". 124b")
	(modify-syntax-entry ?* ". 23")
	(modify-syntax-entry ?\n "> b")
    ;; Chars are the same as strings
    (modify-syntax-entry ?' "\"")
    (syntax-table))
  "Syntax table for `tula-mode'.")

(eval-and-compile
  (defconst tula-keywords
    '("if" "for" "case" "run" "trace" "in" "let" "do" "input" "output" "read" "write" "display")))

(defconst tula-highlights
  `((,(regexp-opt tula-keywords 'symbols) . font-lock-keyword-face)))

;;;###autoload
(define-derived-mode tula-mode prog-mode "tula"
  "Major Mode for editing Tula source code."
  :syntax-table tula-mode-syntax-table
  (setq font-lock-defaults '(tula-highlights))
  (setq-local comment-start "// "))

;;;###autoload
(add-to-list 'auto-mode-alist '("\\.tula\\'" . tula-mode))

(provide 'tula-mode)

;;; tula-mode.el ends here
