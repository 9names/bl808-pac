#[doc = "Register `sd_block_gap_ctrl` reader"]
pub struct R(crate::R<SD_BLOCK_GAP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_BLOCK_GAP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SD_BLOCK_GAP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SD_BLOCK_GAP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_block_gap_ctrl` writer"]
pub struct W(crate::W<SD_BLOCK_GAP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_BLOCK_GAP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SD_BLOCK_GAP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SD_BLOCK_GAP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `stop_at_block_gap_req` reader - "]
pub type STOP_AT_BLOCK_GAP_REQ_R = crate::BitReader<bool>;
#[doc = "Field `stop_at_block_gap_req` writer - "]
pub type STOP_AT_BLOCK_GAP_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_BLOCK_GAP_CTRL_SPEC, bool, O>;
#[doc = "Field `cont_req` reader - "]
pub type CONT_REQ_R = crate::BitReader<bool>;
#[doc = "Field `cont_req` writer - "]
pub type CONT_REQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_BLOCK_GAP_CTRL_SPEC, bool, O>;
#[doc = "Field `rd_wait_ctl` reader - "]
pub type RD_WAIT_CTL_R = crate::BitReader<bool>;
#[doc = "Field `rd_wait_ctl` writer - "]
pub type RD_WAIT_CTL_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_BLOCK_GAP_CTRL_SPEC, bool, O>;
#[doc = "Field `int_blk_gap` reader - "]
pub type INT_BLK_GAP_R = crate::BitReader<bool>;
#[doc = "Field `int_blk_gap` writer - "]
pub type INT_BLK_GAP_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_BLOCK_GAP_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_7_4` reader - "]
pub type RESERVED_7_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `w_card_int` reader - "]
pub type W_CARD_INT_R = crate::BitReader<bool>;
#[doc = "Field `w_card_int` writer - "]
pub type W_CARD_INT_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_BLOCK_GAP_CTRL_SPEC, bool, O>;
#[doc = "Field `w_insertion` reader - "]
pub type W_INSERTION_R = crate::BitReader<bool>;
#[doc = "Field `w_insertion` writer - "]
pub type W_INSERTION_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, SD_BLOCK_GAP_CTRL_SPEC, bool, O>;
#[doc = "Field `w_removal` reader - "]
pub type W_REMOVAL_R = crate::BitReader<bool>;
#[doc = "Field `w_removal` writer - "]
pub type W_REMOVAL_W<'a, const O: u8> = crate::BitWriter<'a, u16, SD_BLOCK_GAP_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_15_11` reader - "]
pub type RESERVED_15_11_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop_at_block_gap_req(&self) -> STOP_AT_BLOCK_GAP_REQ_R {
        STOP_AT_BLOCK_GAP_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cont_req(&self) -> CONT_REQ_R {
        CONT_REQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rd_wait_ctl(&self) -> RD_WAIT_CTL_R {
        RD_WAIT_CTL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn int_blk_gap(&self) -> INT_BLK_GAP_R {
        INT_BLK_GAP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn reserved_7_4(&self) -> RESERVED_7_4_R {
        RESERVED_7_4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn w_card_int(&self) -> W_CARD_INT_R {
        W_CARD_INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn w_insertion(&self) -> W_INSERTION_R {
        W_INSERTION_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn w_removal(&self) -> W_REMOVAL_R {
        W_REMOVAL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15"]
    #[inline(always)]
    pub fn reserved_15_11(&self) -> RESERVED_15_11_R {
        RESERVED_15_11_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn stop_at_block_gap_req(&mut self) -> STOP_AT_BLOCK_GAP_REQ_W<0> {
        STOP_AT_BLOCK_GAP_REQ_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cont_req(&mut self) -> CONT_REQ_W<1> {
        CONT_REQ_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wait_ctl(&mut self) -> RD_WAIT_CTL_W<2> {
        RD_WAIT_CTL_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn int_blk_gap(&mut self) -> INT_BLK_GAP_W<3> {
        INT_BLK_GAP_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn w_card_int(&mut self) -> W_CARD_INT_W<8> {
        W_CARD_INT_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn w_insertion(&mut self) -> W_INSERTION_W<9> {
        W_INSERTION_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn w_removal(&mut self) -> W_REMOVAL_W<10> {
        W_REMOVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Gap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_block_gap_ctrl](index.html) module"]
pub struct SD_BLOCK_GAP_CTRL_SPEC;
impl crate::RegisterSpec for SD_BLOCK_GAP_CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sd_block_gap_ctrl::R](R) reader structure"]
impl crate::Readable for SD_BLOCK_GAP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_block_gap_ctrl::W](W) writer structure"]
impl crate::Writable for SD_BLOCK_GAP_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets sd_block_gap_ctrl to value 0"]
impl crate::Resettable for SD_BLOCK_GAP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
