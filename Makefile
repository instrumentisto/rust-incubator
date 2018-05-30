###############################
# Common defaults/definitions #
###############################

comma := ,

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)




######################
# Project parameters #
######################

MAINLINE_BRANCH := master
CURRENT_BRANCH := $(shell git branch | grep \* | cut -d ' ' -f2)




###########
# Aliases #
###########

squash: git.squash




################
# Git commands #
################

# Squash changes of the current Git branch onto another Git branch.
#
# WARNING: You must merge 'onto' branch in the current branch before squash!
#
# Usage:
#	make git.squash [onto=(<mainline-git-branch>|<git-branch>)]
#	                [del=(no|yes)]
#	                [upstream=(origin|<git-remote>)]

onto ?= $(MAINLINE_BRANCH)
upstream ?= origin

git.squash:
ifeq ($(CURRENT_BRANCH),$(onto))
	@echo "--> Current branch is '$(onto)' already" && false
endif
	git checkout $(onto)
	git branch -m $(CURRENT_BRANCH) orig-$(CURRENT_BRANCH)
	git checkout -b $(CURRENT_BRANCH)
	git branch --set-upstream-to $(upstream)/$(CURRENT_BRANCH)
	git merge --squash orig-$(CURRENT_BRANCH)
ifeq ($(del),yes)
	git branch -d orig-$(CURRENT_BRANCH)
endif




##################
# .PHONY section #
##################

.PHONY: squash \
        git.squash
