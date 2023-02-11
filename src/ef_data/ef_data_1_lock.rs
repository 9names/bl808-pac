#[doc = "Register `ef_data_1_lock` reader"]
pub struct R(crate::R<EF_DATA_1_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_DATA_1_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_DATA_1_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_DATA_1_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_data_1_lock` writer"]
pub struct W(crate::W<EF_DATA_1_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_DATA_1_LOCK_SPEC>;
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
impl From<crate::W<EF_DATA_1_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_DATA_1_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wr_lock_rsvd_1` reader - "]
pub type WR_LOCK_RSVD_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `wr_lock_rsvd_1` writer - "]
pub type WR_LOCK_RSVD_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EF_DATA_1_LOCK_SPEC, u16, u16, 15, O>;
#[doc = "Field `wr_lock_key_slot_4` reader - "]
pub type WR_LOCK_KEY_SLOT_4_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_4` writer - "]
pub type WR_LOCK_KEY_SLOT_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_5` reader - "]
pub type WR_LOCK_KEY_SLOT_5_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_5` writer - "]
pub type WR_LOCK_KEY_SLOT_5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_6` reader - "]
pub type WR_LOCK_KEY_SLOT_6_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_6` writer - "]
pub type WR_LOCK_KEY_SLOT_6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_7` reader - "]
pub type WR_LOCK_KEY_SLOT_7_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_7` writer - "]
pub type WR_LOCK_KEY_SLOT_7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_8` reader - "]
pub type WR_LOCK_KEY_SLOT_8_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_8` writer - "]
pub type WR_LOCK_KEY_SLOT_8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_9` reader - "]
pub type WR_LOCK_KEY_SLOT_9_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_9` writer - "]
pub type WR_LOCK_KEY_SLOT_9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_key_slot_10` reader - "]
pub type WR_LOCK_KEY_SLOT_10_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_key_slot_10` writer - "]
pub type WR_LOCK_KEY_SLOT_10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_dat_1_rsvd_0` reader - "]
pub type WR_LOCK_DAT_1_RSVD_0_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_dat_1_rsvd_0` writer - "]
pub type WR_LOCK_DAT_1_RSVD_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_dat_1_rsvd_1` reader - "]
pub type WR_LOCK_DAT_1_RSVD_1_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_dat_1_rsvd_1` writer - "]
pub type WR_LOCK_DAT_1_RSVD_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `wr_lock_dat_1_rsvd_2` reader - "]
pub type WR_LOCK_DAT_1_RSVD_2_R = crate::BitReader<bool>;
#[doc = "Field `wr_lock_dat_1_rsvd_2` writer - "]
pub type WR_LOCK_DAT_1_RSVD_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_4` reader - "]
pub type RD_LOCK_KEY_SLOT_4_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_4` writer - "]
pub type RD_LOCK_KEY_SLOT_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_5` reader - "]
pub type RD_LOCK_KEY_SLOT_5_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_5` writer - "]
pub type RD_LOCK_KEY_SLOT_5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_6` reader - "]
pub type RD_LOCK_KEY_SLOT_6_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_6` writer - "]
pub type RD_LOCK_KEY_SLOT_6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_7` reader - "]
pub type RD_LOCK_KEY_SLOT_7_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_7` writer - "]
pub type RD_LOCK_KEY_SLOT_7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_8` reader - "]
pub type RD_LOCK_KEY_SLOT_8_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_8` writer - "]
pub type RD_LOCK_KEY_SLOT_8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_9` reader - "]
pub type RD_LOCK_KEY_SLOT_9_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_9` writer - "]
pub type RD_LOCK_KEY_SLOT_9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
#[doc = "Field `rd_lock_key_slot_10` reader - "]
pub type RD_LOCK_KEY_SLOT_10_R = crate::BitReader<bool>;
#[doc = "Field `rd_lock_key_slot_10` writer - "]
pub type RD_LOCK_KEY_SLOT_10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EF_DATA_1_LOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn wr_lock_rsvd_1(&self) -> WR_LOCK_RSVD_1_R {
        WR_LOCK_RSVD_1_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wr_lock_key_slot_4(&self) -> WR_LOCK_KEY_SLOT_4_R {
        WR_LOCK_KEY_SLOT_4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn wr_lock_key_slot_5(&self) -> WR_LOCK_KEY_SLOT_5_R {
        WR_LOCK_KEY_SLOT_5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn wr_lock_key_slot_6(&self) -> WR_LOCK_KEY_SLOT_6_R {
        WR_LOCK_KEY_SLOT_6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn wr_lock_key_slot_7(&self) -> WR_LOCK_KEY_SLOT_7_R {
        WR_LOCK_KEY_SLOT_7_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn wr_lock_key_slot_8(&self) -> WR_LOCK_KEY_SLOT_8_R {
        WR_LOCK_KEY_SLOT_8_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wr_lock_key_slot_9(&self) -> WR_LOCK_KEY_SLOT_9_R {
        WR_LOCK_KEY_SLOT_9_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wr_lock_key_slot_10(&self) -> WR_LOCK_KEY_SLOT_10_R {
        WR_LOCK_KEY_SLOT_10_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_0(&self) -> WR_LOCK_DAT_1_RSVD_0_R {
        WR_LOCK_DAT_1_RSVD_0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_1(&self) -> WR_LOCK_DAT_1_RSVD_1_R {
        WR_LOCK_DAT_1_RSVD_1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn wr_lock_dat_1_rsvd_2(&self) -> WR_LOCK_DAT_1_RSVD_2_R {
        WR_LOCK_DAT_1_RSVD_2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rd_lock_key_slot_4(&self) -> RD_LOCK_KEY_SLOT_4_R {
        RD_LOCK_KEY_SLOT_4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rd_lock_key_slot_5(&self) -> RD_LOCK_KEY_SLOT_5_R {
        RD_LOCK_KEY_SLOT_5_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rd_lock_key_slot_6(&self) -> RD_LOCK_KEY_SLOT_6_R {
        RD_LOCK_KEY_SLOT_6_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rd_lock_key_slot_7(&self) -> RD_LOCK_KEY_SLOT_7_R {
        RD_LOCK_KEY_SLOT_7_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rd_lock_key_slot_8(&self) -> RD_LOCK_KEY_SLOT_8_R {
        RD_LOCK_KEY_SLOT_8_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn rd_lock_key_slot_9(&self) -> RD_LOCK_KEY_SLOT_9_R {
        RD_LOCK_KEY_SLOT_9_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rd_lock_key_slot_10(&self) -> RD_LOCK_KEY_SLOT_10_R {
        RD_LOCK_KEY_SLOT_10_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_rsvd_1(&mut self) -> WR_LOCK_RSVD_1_W<0> {
        WR_LOCK_RSVD_1_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_4(&mut self) -> WR_LOCK_KEY_SLOT_4_W<15> {
        WR_LOCK_KEY_SLOT_4_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_5(&mut self) -> WR_LOCK_KEY_SLOT_5_W<16> {
        WR_LOCK_KEY_SLOT_5_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_6(&mut self) -> WR_LOCK_KEY_SLOT_6_W<17> {
        WR_LOCK_KEY_SLOT_6_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_7(&mut self) -> WR_LOCK_KEY_SLOT_7_W<18> {
        WR_LOCK_KEY_SLOT_7_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_8(&mut self) -> WR_LOCK_KEY_SLOT_8_W<19> {
        WR_LOCK_KEY_SLOT_8_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_9(&mut self) -> WR_LOCK_KEY_SLOT_9_W<20> {
        WR_LOCK_KEY_SLOT_9_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_key_slot_10(&mut self) -> WR_LOCK_KEY_SLOT_10_W<21> {
        WR_LOCK_KEY_SLOT_10_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_dat_1_rsvd_0(&mut self) -> WR_LOCK_DAT_1_RSVD_0_W<22> {
        WR_LOCK_DAT_1_RSVD_0_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_dat_1_rsvd_1(&mut self) -> WR_LOCK_DAT_1_RSVD_1_W<23> {
        WR_LOCK_DAT_1_RSVD_1_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn wr_lock_dat_1_rsvd_2(&mut self) -> WR_LOCK_DAT_1_RSVD_2_W<24> {
        WR_LOCK_DAT_1_RSVD_2_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_4(&mut self) -> RD_LOCK_KEY_SLOT_4_W<25> {
        RD_LOCK_KEY_SLOT_4_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_5(&mut self) -> RD_LOCK_KEY_SLOT_5_W<26> {
        RD_LOCK_KEY_SLOT_5_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_6(&mut self) -> RD_LOCK_KEY_SLOT_6_W<27> {
        RD_LOCK_KEY_SLOT_6_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_7(&mut self) -> RD_LOCK_KEY_SLOT_7_W<28> {
        RD_LOCK_KEY_SLOT_7_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_8(&mut self) -> RD_LOCK_KEY_SLOT_8_W<29> {
        RD_LOCK_KEY_SLOT_8_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_9(&mut self) -> RD_LOCK_KEY_SLOT_9_W<30> {
        RD_LOCK_KEY_SLOT_9_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn rd_lock_key_slot_10(&mut self) -> RD_LOCK_KEY_SLOT_10_W<31> {
        RD_LOCK_KEY_SLOT_10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_data_1_lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_data_1_lock](index.html) module"]
pub struct EF_DATA_1_LOCK_SPEC;
impl crate::RegisterSpec for EF_DATA_1_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_data_1_lock::R](R) reader structure"]
impl crate::Readable for EF_DATA_1_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_data_1_lock::W](W) writer structure"]
impl crate::Writable for EF_DATA_1_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ef_data_1_lock to value 0"]
impl crate::Resettable for EF_DATA_1_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
